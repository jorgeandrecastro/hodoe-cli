
# HODOE CLI - README


L'outil officiel en ligne de commande pour le réseau HODOE.

Plateforme web : https://hodoe.fr

HODOE CLI permet aux développeurs de systèmes embarqués et aux Doers de 
publier et gérer leurs binaires compilés (ESP32, STM32, RISC-V, RP2350, 
ARM...) directement depuis leur terminal vers le hub HODOE.

Version actuelle : v0.5.1

------------------------------------------------------------------------
0. QUOI DE NEUF DANS LA v0.5.0 ?
------------------------------------------------------------------------

- Lien vers la plateforme : Intégration directe du lien vers https://hodoe.fr 
  dans le CLI et la documentation.
- Support de --help enrichi : Description complète du rôle du CLI et 
  de ses fonctionnalités directement accessible via `hodoe --help`.
- Navigation optimisée : Amélioration de la gestion des routes web et 
  retouche de l'interface graphique mobile/web.
- Téléversement sécurisé : Gestion robuste des Presigned URLs et retours 
  d'erreurs clairs lors de la publication des fichiers binaires.

------------------------------------------------------------------------
1. INSTALLATION
------------------------------------------------------------------------

Via Cargo (Recommandé) :
```bash
  cargo install hodoe-cli
```

Depuis le dépôt Git :
```bash
  git clone https://github.com/jorgeandrecastro/hodoe-cli.git
  cd hodoe-cli
  cargo install --path .
```

------------------------------------------------------------------------
2. UTILISATION
------------------------------------------------------------------------

Étape 1 : Authentification
Récupérez votre jeton utilisateur depuis votre espace sur https://hodoe.fr 
et authentifiez votre terminal :

```bash
   hodoe login --token <VOTRE_USER_ID>
   ```

(Vos identifiants de session sont conservés dans ~/.hodoe/config.json)

Étape 2 : Publication d'un binaire
Rendez-vous dans le dossier de votre projet compilé et exécutez :
```

  hodoe binary <NOM_PROJET> push --file <CHEMIN> --arch <ARCHITECTURE>
  ```

Exemples :
```bash
  hodoe binary firmware_capteur push --file ./build/firmware.uf2 --arch rp2350
  hodoe binary weather_station push --file ./target/release/app.bin --arch esp32 --github https://github.com/user/repo --description "Station météo IoT"
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

- Sécurité : Utilisation d'URLs d'envoi pré-signées (Presigned URLs)
  générées à la volée par le backend Axum, sans stockage de clés 
  privées Supabase côté client.
- Téléversement optimisé : Envoi physique direct et sécurisé en HTTP PUT 
  vers Supabase Storage avec suivi de progression en temps réel (indicatif).
- Consommation mémoire minimale (Zero-RAM overhead) grâce à l'écosystème 
  Rust, Tokio et au streaming HTTP.
- Enregistrement atomique : Enregistrement instantané et sécurisé des 
  métadonnées du binaire sur la base de données de l'API HODOE,
  consultables sur https://hodoe.fr.

------------------------------------------------------------------------
5. LICENCE & AUTEUR
------------------------------------------------------------------------

Licence : GPL-2.0-or-later
Auteur  : Jorge Andre Castro <georgeandrec@gmail.com>
Site    : https://hodoe.fr
========================================================================