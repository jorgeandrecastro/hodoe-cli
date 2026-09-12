===================================================================
     HODOE CLI - README
===================================================================

HODOE CLI est l'outil officiel en ligne de commande pour publier et
gérer vos binaires embarqués (ESP32, STM32, RISC-V, ARM) directement
depuis votre terminal vers la plateforme HODOE.

------------------------------------------------------------------------
1. INSTALLATION
------------------------------------------------------------------------

Via Cargo (Recommandé) :
$ cargo install hodoe-cli

Depuis le dépôt Git :
$ cargo install --git https://github.com/jorgeandrecastro/hodoe-cli

------------------------------------------------------------------------
2. UTILISATION
------------------------------------------------------------------------

Étape 1 : Authentification
Récupérez votre jeton utilisateur depuis l'application mobile/web HODOE
(dans l'onglet "Hub Binaires / CLI") et connectez-vous :

$ hodoe login --token <VOTRE_USER_ID>

Étape 2 : Publication d'un binaire
Rendez-vous dans le dossier de votre projet compilé et exécutez :

$ hodoe binary <nom_du_projet> push --file <chemin_fichier> --arch <architecture>

Exemples :
$ hodoe binary firmware_capteur push --file ./build/firmware.bin --arch esp32
$ hodoe binary kernel_riscv push --file ./target/release/app.elf --arch riscv --github https://github.com/user/repo

------------------------------------------------------------------------
3. OPTIONS DISPONIBLES
------------------------------------------------------------------------

Commandes de publication :
  -f, --file <PATH>         Chemin vers le fichier binaire (.bin, .hex, .elf)
  -a, --arch <ARCH>         Architecture cible (esp32, stm32, riscv, arm)
  -g, --github <URL>        (Optionnel) Lien vers le dépôt GitHub source
  -d, --description <DESC>  (Optionnel) Description du projet

------------------------------------------------------------------------
4. LICENCE
------------------------------------------------------------------------

Sous licence GPL-2.0-or-later.
Développé par Jorge Andre Castro <georgeandrec@gmail.com>.
========================================================================