use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "hodoe",
    version = "0.1.0",
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
        /// Chemin d'accès au fichier binaire (.bin, .hex, etc.)
        #[arg(short, long)]
        file: PathBuf,

        /// Architecture cible (ex: esp32, stm32, riscv, arm)
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
        .unwrap_or_else(|| "https://hodoe-api.up.railway.app".to_string());

    match cli.command {
        Commands::Login { token, url } => {
            println!("🔒 Configuration de la session HODOE...");
            config.api_key = Some(token);
            if let Some(custom_url) = url {
                config.api_url = Some(custom_url);
            }
            save_config(&config);
            println!("✅ Authentification réussie ! Identifiants enregistrés dans ~/.hodoe/config.json");
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
                        eprintln!("❌ Vous n'êtes pas connecté.");
                        eprintln!("   Exécutez d'abord : hodoe login --token <VOTRE_USER_ID>");
                        return Ok(());
                    }
                };

                if !file.exists() {
                    eprintln!("❌ Erreur : Le fichier spécifié '{:?}' est introuvable.", file);
                    return Ok(());
                }

                let file_name = file
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("binary.bin");

                println!("📦 Envoi du binaire '{}' ({}) vers HODOE...", name, arch);

                let storage_url = format!(
                    "https://gkcpqctfndmlbfulqzfq.supabase.co/storage/v1/object/public/binaries/{}",
                    file_name
                );

                let client = reqwest::Client::new();
                let payload = serde_json::json!({
                    "author_id": api_key,
                    "name": name,
                    "description": description,
                    "target_architecture": arch,
                    "file_url": storage_url,
                    "github_url": github
                });

                let response = client
                    .post(format!("{}/api/v1/binaries", api_url))
                    .json(&payload)
                    .send()
                    .await?;

                if response.status().is_success() {
                    println!("🚀 Le binaire '{}' a été publié avec succès sur le hub HODOE !", name);
                } else {
                    let err_msg = response.text().await?;
                    eprintln!("❌ Échec de la publication : {}", err_msg);
                }
            }
        },
    }

    Ok(())
}