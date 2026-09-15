use fltk::enums::Color;

// ============================================================================
// ÉTUDE ET COMBINAISON DE COULEURS HAUTE LISIBILITÉ (WCAG AAA - Dark Mode)
// ============================================================================

// Arrière-plans hiérarchisés pour un contraste naturel et une profondeur visuelle
pub const COLOR_BG: Color = Color::from_rgb(15, 23, 42);          // #0f172a (Fond principal - Slate sombre)
pub const COLOR_PANEL: Color = Color::from_rgb(30, 41, 59);       // #1e293b (Panneaux et conteneurs - Slate 800)
pub const COLOR_CARD: Color = Color::from_rgb(40, 53, 72);        // #283548 (Cartes de télémétrie - Élevé)
pub const COLOR_INPUT: Color = Color::from_rgb(11, 17, 32);       // #0b1120 (Fond des champs de saisie - Creusé)
#[allow(dead_code)]
pub const COLOR_BORDER: Color = Color::from_rgb(51, 65, 85);      // #334155 (Bordures techniques)

// Typographie à très haut contraste
pub const COLOR_TEXT_WHITE: Color = Color::from_rgb(255, 255, 255);// #ffffff (Blanc pur - Titres & valeurs clés)
pub const COLOR_TEXT_MAIN: Color = Color::from_rgb(241, 245, 249); // #f1f5f9 (Blanc cassé doux - Ratio 13.5:1)
pub const COLOR_TEXT_MUTED: Color = Color::from_rgb(148, 163, 184);// #94a3b8 (Gris ardoise lisible - Ratio 5.8:1)
pub const COLOR_TEXT_ACCENT: Color = Color::from_rgb(56, 189, 248);// #38bdf8 (Cyan vif - Repères techniques)

// Accents sémantiques et états
pub const COLOR_EMERALD: Color = Color::from_rgb(16, 185, 129);    // #10b981 (Bouton Démarrer / Hotspot Actif)
pub const COLOR_EMERALD_TEXT: Color = Color::from_rgb(52, 211, 153);// #34d399 (Texte actif ultra-lumineux)
pub const COLOR_RED: Color = Color::from_rgb(220, 38, 38);         // #dc2626 (Bouton Arrêter / Hotspot Inactif)
pub const COLOR_RED_TEXT: Color = Color::from_rgb(248, 113, 113);  // #f87171 (Texte inactif lumineux)
pub const COLOR_BLUE_BTN: Color = Color::from_rgb(37, 99, 235);    // #2563eb (Bouton Appliquer)
pub const COLOR_AMBER: Color = Color::from_rgb(245, 158, 11);      // #f59e0b (Statut en attente)
