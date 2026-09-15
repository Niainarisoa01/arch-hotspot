use crate::theme::*;
use fltk::{
    button::Button,
    draw,
    enums::{Align, Color, ColorDepth, Event, Font, FrameType, Key},
    frame::Frame,
    image::{PngImage, RgbImage},
    prelude::*,
    window::DoubleWindow,
};
use qrcode::{Color as QrColor, QrCode};

static APP_ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

/// Échappe les caractères réservés dans la chaîne Wi-Fi selon la norme Wi-Fi Alliance
fn escape_wifi_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' | ';' | ',' | ':' | '"' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

/// Génère une image RGB ultra-nette et contrastée du QR code Wi-Fi
pub fn generate_qr_image(ssid: &str, password: &str, target_size: i32) -> Option<RgbImage> {
    // Format officiel Wi-Fi Alliance standardisé :
    // WIFI:S:<SSID>;T:WPA;P:<PASSWORD>;;
    let escaped_ssid = escape_wifi_str(ssid);
    let payload = if password.is_empty() {
        format!("WIFI:S:{};T:nopass;;", escaped_ssid)
    } else {
        let escaped_pwd = escape_wifi_str(password);
        format!("WIFI:S:{};T:WPA;P:{};;", escaped_ssid, escaped_pwd)
    };

    let code = QrCode::new(payload.as_bytes()).ok()?;
    let qr_width = code.width();
    let colors = code.to_colors();

    // Bordure blanche (zone de silence / quiet zone de 2 modules indispensable aux scanners)
    let quiet_zone = 2;
    let full_modules = qr_width + (quiet_zone * 2);
    let scale = (target_size / full_modules as i32).max(1);
    let final_dim = (full_modules as i32) * scale;

    let mut rgb_data = Vec::with_capacity((final_dim * final_dim * 3) as usize);

    for y in 0..final_dim {
        let mod_y = (y / scale) - quiet_zone as i32;
        for x in 0..final_dim {
            let mod_x = (x / scale) - quiet_zone as i32;

            let is_dark = if mod_x >= 0 && mod_x < qr_width as i32 && mod_y >= 0 && mod_y < qr_width as i32 {
                colors[(mod_y as usize) * qr_width + (mod_x as usize)] == QrColor::Dark
            } else {
                false // Zone de bordure blanche
            };

            if is_dark {
                // Noir pur (#000000) pour une lisibilité optique maximale par caméra
                rgb_data.extend_from_slice(&[0, 0, 0]);
            } else {
                // Blanc pur (#ffffff)
                rgb_data.extend_from_slice(&[255, 255, 255]);
            }
        }
    }

    RgbImage::new(&rgb_data, final_dim, final_dim, ColorDepth::Rgb8).ok()
}

/// Affiche une fenêtre modale moderne et fluide avec le QR code Wi-Fi prêt à scanner
pub fn show_qr_modal(ssid: &str, password: &str) {
    let win_w = 420;
    let win_h = 530;
    let mut dialog = DoubleWindow::default()
        .with_size(win_w, win_h)
        .with_label("Connexion Wi-Fi par QR Code");
    dialog.set_color(COLOR_BG);
    dialog.make_modal(true);

    if let Ok(icon) = PngImage::from_data(APP_ICON_BYTES) {
        dialog.set_icon(Some(icon));
    }

    // 1. Titre et sous-titre
    let mut title = Frame::new(20, 18, win_w - 40, 24, "SCANNER POUR SE CONNECTER");
    title.set_label_color(COLOR_TEXT_WHITE);
    title.set_label_font(Font::HelveticaBold);
    title.set_label_size(15);
    title.set_align(Align::Center);

    let mut subtitle = Frame::new(20, 42, win_w - 40, 18, "Pointez l'appareil photo de votre smartphone");
    subtitle.set_label_color(COLOR_TEXT_MUTED);
    subtitle.set_label_font(Font::Helvetica);
    subtitle.set_label_size(11);
    subtitle.set_align(Align::Center);

    // 2. Zone blanche pour le QR Code
    let qr_box_size = 260;
    let qr_x = (win_w - qr_box_size) / 2;
    let qr_y = 68;

    let mut qr_frame = Frame::new(qr_x, qr_y, qr_box_size, qr_box_size, None);
    qr_frame.set_frame(FrameType::FlatBox);
    qr_frame.set_color(Color::White);

    if let Some(mut img) = generate_qr_image(ssid, password, qr_box_size) {
        let img_w = img.width();
        let img_h = img.height();
        qr_frame.draw(move |f| {
            draw::draw_box(FrameType::FlatBox, f.x(), f.y(), f.w(), f.h(), Color::White);
            let offset_x = (f.w() - img_w) / 2;
            let offset_y = (f.h() - img_h) / 2;
            img.draw(f.x() + offset_x, f.y() + offset_y, img_w, img_h);
        });
    }

    // 3. Carte d'informations du réseau sous le QR Code
    let info_y = qr_y + qr_box_size + 16;
    let mut info_card = Frame::new(30, info_y, win_w - 60, 78, None);
    info_card.set_frame(FrameType::FlatBox);
    info_card.set_color(COLOR_CARD);

    let mut ssid_disp = Frame::new(40, info_y + 10, win_w - 80, 22, None);
    ssid_disp.set_label(&format!("Réseau (SSID) : {}", ssid));
    ssid_disp.set_label_color(COLOR_TEXT_WHITE);
    ssid_disp.set_label_font(Font::HelveticaBold);
    ssid_disp.set_label_size(13);
    ssid_disp.set_align(Align::Center);

    let mut pwd_disp = Frame::new(40, info_y + 38, win_w - 80, 22, None);
    if password.is_empty() {
        pwd_disp.set_label("Sécurité : Réseau ouvert (sans mot de passe)");
        pwd_disp.set_label_color(COLOR_TEXT_MUTED);
        pwd_disp.set_label_font(Font::Helvetica);
    } else {
        pwd_disp.set_label(&format!("Mot de passe : {}", password));
        pwd_disp.set_label_color(COLOR_TEXT_ACCENT);
        pwd_disp.set_label_font(Font::CourierBold);
    }
    pwd_disp.set_label_size(13);
    pwd_disp.set_align(Align::Center);

    // 4. Bouton de fermeture
    let btn_y = info_y + 78 + 14;
    let mut btn_close = Button::new(30, btn_y, win_w - 60, 42, "FERMER");
    btn_close.set_color(COLOR_PANEL);
    btn_close.set_label_color(COLOR_TEXT_WHITE);
    btn_close.set_label_font(Font::HelveticaBold);
    btn_close.set_label_size(13);

    let mut d_close = dialog.clone();
    btn_close.set_callback(move |_| {
        d_close.hide();
    });

    // Fermeture avec la touche Échap
    let mut d_esc = dialog.clone();
    dialog.handle(move |_, ev| match ev {
        Event::KeyDown => {
            if fltk::app::event_key() == Key::Escape {
                d_esc.hide();
                true
            } else {
                false
            }
        }
        _ => false,
    });

    dialog.end();
    dialog.show();
}
