#!/usr/bin/env bash
# Installation locale du lanceur et de l'icône
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "==> Installation du binaire dans ~/.local/bin/..."
mkdir -p "$HOME/.local/bin"
cp "$DIR/target/release/arch-hotspot-gui" "$HOME/.local/bin/arch-hotspot-gui"
chmod +x "$HOME/.local/bin/arch-hotspot-gui"

echo "==> Installation de l'icône dans ~/.local/share/icons/..."
mkdir -p "$HOME/.local/share/icons/hicolor/256x256/apps"
cp "$DIR/assets/icon.png" "$HOME/.local/share/icons/hicolor/256x256/apps/arch-hotspot.png"

echo "==> Installation du fichier .desktop..."
mkdir -p "$HOME/.local/share/applications"
cp "$DIR/arch-hotspot.desktop" "$HOME/.local/share/applications/arch-hotspot.desktop"
chmod +x "$HOME/.local/share/applications/arch-hotspot.desktop"

# Copie sur le Bureau si présent
if [ -d "$HOME/Desktop" ]; then
    cp "$HOME/.local/share/applications/arch-hotspot.desktop" "$HOME/Desktop/"
    chmod +x "$HOME/Desktop/arch-hotspot.desktop"
    gio set "$HOME/Desktop/arch-hotspot.desktop" metadata::trusted true 2>/dev/null || true
fi
if [ -d "$HOME/Bureau" ]; then
    cp "$HOME/.local/share/applications/arch-hotspot.desktop" "$HOME/Bureau/"
    chmod +x "$HOME/Bureau/arch-hotspot.desktop"
    gio set "$HOME/Bureau/arch-hotspot.desktop" metadata::trusted true 2>/dev/null || true
fi

update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
echo "==> Installation terminée avec succès !"
