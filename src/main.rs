use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

#[derive(Parser)]
#[command(
    name = "hodoe",
    version = "0.2.0",
    about = "Outil en ligne de commande HODOE pour le déploiement de binaires embarqués",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connexion à votre compte HODOE via votre jeton utilisateur
    Login {
        /// Votre jeton d'authentification utilisateur HODOE
        #[arg(short, long)]
        token: String,

        /// URL personnalisée du serveur API backend (optionnel)
        #[arg(short, long)]
        url: Option<String>,
    },
    /// Gestion et publication des binaires embarqués
    Binary {
        /// Nom du projet ou du fichier binaire
        name: String,

        #[command(subcommand)]
        action: BinaryActions,
    },
}

#[derive(Subcommand)]
enum BinaryActions {
    /// Déployer et publier un binaire compilé sur le hub HODOE
    Push {
        /// Chemin d'accès au fichier binaire (.bin, .hex, .uf2, etc.)
        #[arg(short, long)]
        file: PathBuf,

        /// Architecture cible (ex: esp32, stm32, riscv, rp2350)
        #[arg(short, long)]
        arch: String,

        /// Lien vers le dépôt GitHub source (optionnel)
        #[arg(short, long)]
        github: Option<String>,

        /// Description détaillée du binaire (optionnel)
        #[arg(short, long)]
        description: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    api_key: Option<String>,
    api_url: Option<String>,
}

#[derive(Deserialize)]
struct PresignResponse {
    upload_url: String,
    public_url: String,
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".hodoe");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

fn load_config() -> Config {
    let path = get_config_path();
    if let Ok(content) = fs::read_to_string(path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}

fn save_config(config: &Config) {
    let path = get_config_path();
    if let Ok(content) = serde_json::to_string_pretty(config) {
        fs::write(path, content).ok();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = load_config();

    let api_url = config
        .api_url
        .clone()
        .unwrap_or_else(|| "https://hodoe-api.onrender.com".to_string());

    match cli.command {
        Commands::Login { token, url } => {
            println!("{}", style("🔒 Configuration de la session HODOE...").cyan().bold());
            config.api_key = Some(token);
            if let Some(custom_url) = url {
                config.api_url = Some(custom_url);
            }
            save_config(&config);
            println!("{}", style("✅ Authentification réussie ! Identifiants enregistrés dans ~/.hodoe/config.json").green().bold());
        }
        Commands::Binary { name, action } => match action {
            BinaryActions::Push {
                file,
                arch,
                github,
                description,
            } => {
                let api_key = match &config.api_key {
                    Some(key) => key,
                    None => {
                        eprintln!("{}", style("❌ Vous n'êtes pas connecté.").red().bold());
                        eprintln!("   Exécutez d'abord : hodoe login --token <VOTRE_USER_ID>");
                        return Ok(());
                    }
                };

                if !file.exists() {
                    eprintln!(
                        "{}",
                        style(format!("❌ Erreur : Le fichier '{:?}' est introuvable.", file)).red().bold()
                    );
                    return Ok(());
                }

                let tokio_file = tokio::fs::File::open(&file).await?;
                let file_size = tokio_file.metadata().await?.len();

                let file_name = file
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("binary.bin");

                let client = Client::new();

                // 1. Obtention de l'URL d'upload temporaire (Presigned URL) auprès du backend Axum
                println!("{}", style("🔑 Demande d'autorisation d'envoi au backend...").cyan());

                let presign_res = client
                    .post(format!("{}/api/v1/binaries/presign", api_url))
                    .json(&serde_json::json!({ "filename": file_name }))
                    .send()
                    .await?;

                if !presign_res.status().is_success() {
                    let err = presign_res.text().await?;
                    eprintln!("{}", style(format!("❌ Erreur d'autorisation backend : {}", err)).red());
                    return Ok(());
                }

                let presign_data: PresignResponse = presign_res.json().await?;

                // 2. Téléversement du binaire vers Supabase via la Presigned URL (requête PUT)
                println!(
                    "{}",
                    style(format!("📦 Téléversement du binaire '{}' ({}, {} octets)...", name, arch, file_size)).blue().bold()
                );

                let pb = ProgressBar::new(file_size);
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                        .progress_chars("#>-"),
                );

                let stream = ReaderStream::new(tokio_file);
                let body = reqwest::Body::wrap_stream(stream);

                let upload_res = client
                    .put(&presign_data.upload_url)
                    .header("Content-Type", "application/octet-stream")
                    .body(body)
                    .send()
                    .await?;

                if upload_res.status().is_success() {
                    pb.finish_with_message("✨ Téléversement physique vers Supabase réussi !");
                } else {
                    pb.abandon();
                    let err = upload_res.text().await?;
                    eprintln!("{}", style(format!("❌ Échec du téléversement : {}", err)).red());
                    return Ok(());
                }

                // 3. Enregistrement des métadonnées sur le backend Axum
                println!("{}", style("📡 Enregistrement des métadonnées sur HODOE...").magenta().bold());

                let payload = serde_json::json!({
                    "author_id": api_key,
                    "name": name,
                    "description": description,
                    "target_architecture": arch,
                    "file_url": presign_data.public_url,
                    "github_url": github
                });

                let response = client
                    .post(format!("{}/api/v1/binaries", api_url))
                    .json(&payload)
                    .send()
                    .await?;

                if response.status().is_success() {
                    println!(
                        "{}",
                        style(format!("🚀 Le binaire '{}' a été publié avec succès !", name)).green().bold()
                    );
                } else {
                    let err_msg = response.text().await?;
                    eprintln!("{}", style(format!("❌ Échec de l'enregistrement API : {}", err_msg)).red());
                }
            }
        },
    }

    Ok(())
}