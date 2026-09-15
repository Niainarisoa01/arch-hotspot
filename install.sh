#!/usr/bin/env bash
# Installation locale du lanceur et de l'icône
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Détermination du dossier d'installation du binaire
if [ -d "$HOME/.cargo/bin" ]; then
    BIN_DIR="$HOME/.cargo/bin"
else
    BIN_DIR="$HOME/.local/bin"
fi

mkdir -p "$BIN_DIR"
mkdir -p "$HOME/.local/bin"
echo "==> Installation du binaire dans $BIN_DIR..."
cp "$DIR/target/release/arch-hotspot-gui" "$BIN_DIR/arch-hotspot-gui"
cp "$DIR/target/release/arch-hotspot-gui" "$HOME/.local/bin/arch-hotspot-gui" 2>/dev/null || true
chmod +x "$BIN_DIR/arch-hotspot-gui"

echo "==> Installation de l'icône dans ~/.local/share/icons/..."
ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"
mkdir -p "$ICON_DIR"
cp "$DIR/assets/icon.png" "$ICON_DIR/arch-hotspot.png"

echo "==> Génération du fichier .desktop avec chemin absolu..."
APPS_DIR="$HOME/.local/share/applications"
mkdir -p "$APPS_DIR"

cat << DESKTOP_EOF > "$APPS_DIR/arch-hotspot.desktop"
[Desktop Entry]
Name=Arch Hotspot Wi-Fi
GenericName=Partage de Connexion
Comment=Gestionnaire de Point d'accès Wi-Fi et surveillance des clients (Hotspot)
Exec=$BIN_DIR/arch-hotspot-gui
Icon=$ICON_DIR/arch-hotspot.png
Terminal=false
Type=Application
Categories=Network;System;
Keywords=wifi;hotspot;partage;connexion;arch;
StartupNotify=true
DESKTOP_EOF

chmod +x "$APPS_DIR/arch-hotspot.desktop"

# Copie sur le Bureau si présent
if [ -d "$HOME/Desktop" ]; then
    cp "$APPS_DIR/arch-hotspot.desktop" "$HOME/Desktop/"
    chmod +x "$HOME/Desktop/arch-hotspot.desktop"
    gio set "$HOME/Desktop/arch-hotspot.desktop" metadata::trusted true 2>/dev/null || true
fi
if [ -d "$HOME/Bureau" ]; then
    cp "$APPS_DIR/arch-hotspot.desktop" "$HOME/Bureau/"
    chmod +x "$HOME/Bureau/arch-hotspot.desktop"
    gio set "$HOME/Bureau/arch-hotspot.desktop" metadata::trusted true 2>/dev/null || true
fi

update-desktop-database "$APPS_DIR" 2>/dev/null || true
which kbuildsycoca6 >/dev/null 2>&1 && kbuildsycoca6 2>/dev/null || true

echo "==> Installation terminée avec succès ! Le raccourci est actif sur votre Bureau et dans le menu d'applications."
