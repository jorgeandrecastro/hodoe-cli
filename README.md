
# HODOE CLI - README


HODOE CLI est l'outil officiel en ligne de commande pour publier et
gérer vos binaires embarqués (ESP32, STM32, RISC-V, RP2350, ARM)
directement depuis votre terminal vers le hub HODOE.

------------------------------------------------------------------------
1. INSTALLATION
------------------------------------------------------------------------

Via Cargo (Recommandé) :
  $ cargo install hodoe-cli

Depuis le dépôt Git :
```bash
 - $ git clone https://github.com/jorgeandrecastro/hodoe-cli.git
 - $ cd hodoe-cli
 - $ cargo install --path .
 ```

------------------------------------------------------------------------
2. UTILISATION
------------------------------------------------------------------------

Étape 1 : Authentification
Récupérez votre jeton utilisateur depuis la plateforme HODOE et
authentifiez votre terminal :

```bash
  $ hodoe login --token <VOTRE_USER_ID>
```

(Vos identifiants de session sont conservés dans ~/.hodoe/config.json)

Étape 2 : Publication d'un binaire
Rendez-vous dans le dossier de votre projet compilé et exécutez :

  ```bash
  $ hodoe binary <NOM_PROJET> push --file <CHEMIN> --arch <ARCHITECTURE>
  ```

Exemples :
```bash
  $ hodoe binary firmware_capteur push --file ./build/firmware.uf2 --arch rp2350
  $ hodoe binary weather_station push --file ./target/release/app.bin --arch esp32 --github https://github.com/user/repo --description "Station météo IoT"
  ```

------------------------------------------------------------------------
3. OPTIONS DE LA COMMANDE PUSH
------------------------------------------------------------------------

  -f, --file <PATH>         Chemin vers le fichier binaire (.uf2, .bin, .hex, .elf) [Obligatoire]
  -a, --arch <ARCH>         Architecture cible (esp32, stm32, rp2350, riscv, arm)   [Obligatoire]
  -g, --github <URL>        (Optionnel) Lien vers le dépôt GitHub source
  -d, --description <DESC>  (Optionnel) Description synthétique du firmware

------------------------------------------------------------------------
4. FONCTIONNALITÉS & ARCHITECTURE
------------------------------------------------------------------------

- Téléversement en streaming direct vers Supabase Storage avec affichage
  de la barre de progression en temps réel (indicatif).
- Consommation mémoire minimale (Zero-RAM overhead) grâce à Tokio.
- Validation instantanée des métadonnées sur le serveur backend Axum/Render.

------------------------------------------------------------------------
5. LICENCE & AUTEUR
------------------------------------------------------------------------

Sous licence GPL-2.0-or-later.
Développé par Jorge Andre Castro <georgeandrec@gmail.com>.
========================================================================