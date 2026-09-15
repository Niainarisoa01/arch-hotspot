#!/usr/bin/env bash
# Script de lancement pour Arch Hotspot GUI
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/target/release/arch-hotspot-gui" "$@"
