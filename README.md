# Arch Hotspot GUI (Rust) 🌐

Une application de bureau native, légère et moderne en **Rust** (FLTK) pour gérer et surveiller facilement le partage de connexion (Point d'accès / Hotspot Wi-Fi) sous **Arch Linux**.

---

## Fonctionnalités

* **Tableau de bord en temps réel :**
  * Statut visuel immédiat : **● ACTIF** (Vert émeraude) ou **○ INACTIF** (Rouge).
  * Affichage de l'interface Wi-Fi (`wlp4s0`), de l'IP du hotspot (`10.42.0.1`), et du nombre d'appareils connectés.
* **Gestion du Hotspot :**
  * Démarrage et arrêt du partage en 1 clic (Bouton d'action unique basculant).
  * Modification dynamique du nom du réseau (**SSID**) et du mot de passe (**WPA2**).
  * Option pour afficher/masquer le mot de passe.
* **Connexion instantanée par QR Code Wi-Fi :**
  * Génération de QR Code standardisé Wi-Fi Alliance (`WIFI:S:<SSID>;T:WPA;P:<KEY>;;`).
  * Modal haute résolution avec zone de silence optique pour scan rapide par smartphone (Android / iOS).
* **Surveillance des clients (Appareils connectés) :**
  * Tableau actualisé automatiquement listant l'adresse IP, l'adresse MAC et l'état de joignabilité de chaque appareil connecté.
* **Journal d'activité :**
  * Console intégrée affichant l'historique timestampé des opérations réseau.
* **Performances & Économie Maximale :**
  * Binaire ultra-compact : **~750 Ko** (LTO fat + opt-level=z + UPX).
  * Consommation RAM : **~0.2% de la RAM** (~30 Mo tampons graphiques X11/Wayland inclus).
  * Empreinte CPU en veille : **~0.0% CPU**.
  * **Zéro processus enfant en arrière-plan** : lecture directe du noyau via `/proc/net/arp`.
  * **Rendu graphique différentiel** : aucun redraw inutile si l'état ne change pas.

---

## Installation & Lancement rapide

### 1. Cloner le dépôt :
```bash
git clone https://github.com/Niainarisoa01/arch-hotspot.git
cd arch-hotspot
```

### 2. Installer le raccourci Bureau et Menu d'applications :
```bash
./install.sh
```

### 3. Lancer directement :
```bash
./run.sh
```

### 4. Ou compiler avec Cargo :
```bash
cargo run --release
```

---

## Architecture des fichiers

* `src/main.rs` : Point d'entrée de l'application.
* `src/gui.rs` : Interface utilisateur moderne FLTK, gestion des événements asynchrones et timers de rafraîchissement.
* `src/qr.rs` : Générateur de QR Code Wi-Fi standardisé et affichage modal haute résolution.
* `src/network.rs` : Interaction avec NetworkManager (`nmcli`) et détection des voisins réseau (`ip neigh`).
* `src/theme.rs` : Palette de couleurs Dark Mode industrielle (WCAG AAA).
* `assets/` : Icônes et logos de l'application (SVG et PNG).
* `install.sh` : Script d'installation automatique du binaire et du lanceur desktop.
