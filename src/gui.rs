use crate::network::{self, ConnectedClient, QuickTickStatus};
use crate::theme::*;
use anyhow::Result;
use fltk::{
    app::{self, Receiver, Sender},
    browser::HoldBrowser,
    button::{Button, CheckButton},
    enums::{Align, Color, Font, FrameType},
    frame::Frame,
    image::PngImage,
    input::Input,
    prelude::*,
    window::DoubleWindow,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

static APP_ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

#[derive(Clone, Debug)]
pub enum AppMsg {
    TogglePower,
    Apply,
    ToggleShowPassword(bool),
    Tick(QuickTickStatus),
    #[allow(dead_code)]
    Log(String),
}

pub fn run_gui() -> Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    // ========================================================================
    // BANNISSEMENT TOTAL DU TEXTE NOIR DANS LE THÈME SOMBRE :
    // Configuration globale des registres de couleurs du moteur graphique FLTK
    // ========================================================================
    app::set_background_color(15, 23, 42);             // #0f172a (Fond principal)
    app::set_background2_color(11, 17, 32);            // #0b1120 (Fond des zones de texte/inputs)
    app::set_foreground_color(248, 250, 252);          // #f8fafc (Texte par défaut : BLANC PUR)
    app::set_selection_color(37, 99, 235);             // #2563eb (Couleur de surbrillance/sélection)
    app::set_color(Color::Foreground, 248, 250, 252);  // Mappage forcé de l'index 0 (Noir -> Blanc pur)
    app::set_visible_focus(false);

    let (sender, receiver): (Sender<AppMsg>, Receiver<AppMsg>) = app::channel();

    let win_w = 760;
    let win_h = 700;
    let mut wind = DoubleWindow::new(100, 80, win_w, win_h, "Partage de Connexion Wi-Fi - Arch Linux");
    wind.set_color(COLOR_BG);

    // Intégration du Logo de l'application dans la barre de titre et la barre des tâches
    if let Ok(icon) = PngImage::from_data(APP_ICON_BYTES) {
        wind.set_icon(Some(icon));
    }

    // 1. EN-TÊTE PRINCIPALE
    let mut header_bg = Frame::new(20, 16, win_w - 40, 52, None);
    header_bg.set_frame(FrameType::FlatBox);
    header_bg.set_color(COLOR_PANEL);

    let mut header_title = Frame::new(36, 24, 400, 36, "ARCH LINUX  |  PARTAGE DE CONNEXION");
    header_title.set_label_color(COLOR_TEXT_WHITE);
    header_title.set_label_font(Font::HelveticaBold);
    header_title.set_label_size(15);
    header_title.set_align(Align::Left | Align::Inside);

    let mut status_badge = Frame::new(win_w - 250, 24, 210, 36, "CHARGEMENT...");
    status_badge.set_label_color(COLOR_AMBER);
    status_badge.set_label_font(Font::HelveticaBold);
    status_badge.set_label_size(13);
    status_badge.set_align(Align::Right | Align::Inside);

    // 2. PANNEAUX DE TÉLÉMÉTRIE (3 Cartes en haut)
    let card_y = 80;
    let card_h = 78;
    let gap = 14;
    let card_w = (win_w - 40 - (gap * 2)) / 3;

    // Carte 1: Interface Wi-Fi
    let mut c1_box = Frame::new(20, card_y, card_w, card_h, None);
    c1_box.set_frame(FrameType::FlatBox);
    c1_box.set_color(COLOR_CARD);

    let mut c1_lbl = Frame::new(32, card_y + 10, card_w - 24, 16, "INTERFACE WI-FI");
    c1_lbl.set_label_color(COLOR_TEXT_MUTED);
    c1_lbl.set_label_font(Font::HelveticaBold);
    c1_lbl.set_label_size(11);
    c1_lbl.set_align(Align::Left | Align::Inside);

    let mut c1_val = Frame::new(32, card_y + 32, card_w - 24, 32, "--");
    c1_val.set_label_color(COLOR_TEXT_ACCENT);
    c1_val.set_label_font(Font::CourierBold);
    c1_val.set_label_size(20);
    c1_val.set_align(Align::Left | Align::Inside);

    // Carte 2: Adresse Passerelle (Gateway IP)
    let c2_x = 20 + card_w + gap;
    let mut c2_box = Frame::new(c2_x, card_y, card_w, card_h, None);
    c2_box.set_frame(FrameType::FlatBox);
    c2_box.set_color(COLOR_CARD);

    let mut c2_lbl = Frame::new(c2_x + 12, card_y + 10, card_w - 24, 16, "IP DU HOTSPOT");
    c2_lbl.set_label_color(COLOR_TEXT_MUTED);
    c2_lbl.set_label_font(Font::HelveticaBold);
    c2_lbl.set_label_size(11);
    c2_lbl.set_align(Align::Left | Align::Inside);

    let mut c2_val = Frame::new(c2_x + 12, card_y + 32, card_w - 24, 32, "--");
    c2_val.set_label_color(COLOR_TEXT_WHITE);
    c2_val.set_label_font(Font::CourierBold);
    c2_val.set_label_size(19);
    c2_val.set_align(Align::Left | Align::Inside);

    // Carte 3: Appareils connectés
    let c3_x = c2_x + card_w + gap;
    let mut c3_box = Frame::new(c3_x, card_y, card_w, card_h, None);
    c3_box.set_frame(FrameType::FlatBox);
    c3_box.set_color(COLOR_CARD);

    let mut c3_lbl = Frame::new(c3_x + 12, card_y + 10, card_w - 24, 16, "CLIENTS CONNECTÉS");
    c3_lbl.set_label_color(COLOR_TEXT_MUTED);
    c3_lbl.set_label_font(Font::HelveticaBold);
    c3_lbl.set_label_size(11);
    c3_lbl.set_align(Align::Left | Align::Inside);

    let mut c3_val = Frame::new(c3_x + 12, card_y + 32, card_w - 24, 32, "0");
    c3_val.set_label_color(COLOR_EMERALD_TEXT);
    c3_val.set_label_font(Font::HelveticaBold);
    c3_val.set_label_size(24);
    c3_val.set_align(Align::Left | Align::Inside);

    // 3. SECTION CONFIGURATION
    let cfg_y = 172;
    let cfg_h = 160;
    let mut cfg_box = Frame::new(20, cfg_y, win_w - 40, cfg_h, None);
    cfg_box.set_frame(FrameType::FlatBox);
    cfg_box.set_color(COLOR_PANEL);

    let mut cfg_title = Frame::new(36, cfg_y + 12, 350, 20, "PARAMÈTRES DU POINT D'ACCÈS");
    cfg_title.set_label_color(COLOR_TEXT_WHITE);
    cfg_title.set_label_font(Font::HelveticaBold);
    cfg_title.set_label_size(12);
    cfg_title.set_align(Align::Left | Align::Inside);

    // Champ SSID
    let mut ssid_lbl = Frame::new(36, cfg_y + 40, 160, 20, "Nom du réseau (SSID) :");
    ssid_lbl.set_label_color(COLOR_TEXT_MUTED);
    ssid_lbl.set_label_font(Font::Helvetica);
    ssid_lbl.set_label_size(12);
    ssid_lbl.set_align(Align::Left | Align::Inside);

    let mut ssid_input = Input::new(36, cfg_y + 64, 310, 36, None);
    ssid_input.set_color(COLOR_INPUT);
    ssid_input.set_text_color(COLOR_TEXT_WHITE);
    ssid_input.set_cursor_color(COLOR_TEXT_ACCENT);
    ssid_input.set_text_font(Font::HelveticaBold);
    ssid_input.set_text_size(14);
    ssid_input.set_value("Niaina");

    // Champ Mot de passe
    let pwd_x = 380;
    let mut pwd_lbl = Frame::new(pwd_x, cfg_y + 40, 200, 20, "Mot de passe (WPA2) :");
    pwd_lbl.set_label_color(COLOR_TEXT_MUTED);
    pwd_lbl.set_label_font(Font::Helvetica);
    pwd_lbl.set_label_size(12);
    pwd_lbl.set_align(Align::Left | Align::Inside);

    let mut pwd_input = Input::new(pwd_x, cfg_y + 64, 324, 36, None);
    pwd_input.set_color(COLOR_INPUT);
    pwd_input.set_text_color(COLOR_TEXT_WHITE);
    pwd_input.set_cursor_color(COLOR_TEXT_ACCENT);
    pwd_input.set_text_font(Font::CourierBold);
    pwd_input.set_text_size(14);
    pwd_input.set_value("12345678");

    // Checkbox afficher mot de passe
    let mut chk_show_pwd = CheckButton::new(pwd_x, cfg_y + 110, 220, 24, "Masquer le mot de passe");
    chk_show_pwd.set_label_color(COLOR_TEXT_MAIN);
    chk_show_pwd.set_label_size(12);
    chk_show_pwd.set_checked(false);

    let s_pwd = sender.clone();
    chk_show_pwd.set_callback(move |btn| {
        s_pwd.send(AppMsg::ToggleShowPassword(btn.is_checked()));
    });

    // 4. BOUTONS D'ACTIONS (Bouton Unique Bascule Démarrer/Arrêter + Bouton Appliquer)
    let btn_y = 344;
    let btn_h = 42;
    let btn_gap = 16;
    let btn_w = (win_w - 40 - btn_gap) / 2; // Largeur parfaitement équilibrée (352 px)
    let btn_apply_x = 20 + btn_w + btn_gap;

    // BOUTON UNIQUE BASCULE (DÉMARRER / ARRÊTER)
    let mut btn_toggle = Button::new(20, btn_y, btn_w, btn_h, "▶  DÉMARRER LE PARTAGE");
    btn_toggle.set_color(COLOR_EMERALD);
    btn_toggle.set_label_color(COLOR_TEXT_WHITE);
    btn_toggle.set_label_font(Font::HelveticaBold);
    btn_toggle.set_label_size(13);

    let s_toggle = sender.clone();
    btn_toggle.set_callback(move |_| {
        s_toggle.send(AppMsg::TogglePower);
    });

    // Bouton APPLIQUER MODIFICATIONS (Même largeur équilibrée)
    let mut btn_apply = Button::new(btn_apply_x, btn_y, btn_w, btn_h, "💾  APPLIQUER MODIFICATIONS");
    btn_apply.set_color(COLOR_BLUE_BTN);
    btn_apply.set_label_color(COLOR_TEXT_WHITE);
    btn_apply.set_label_font(Font::HelveticaBold);
    btn_apply.set_label_size(13);

    let s_apply = sender.clone();
    btn_apply.set_callback(move |_| {
        s_apply.send(AppMsg::Apply);
    });

    // 5. LISTE DES APPAREILS CONNECTÉS (Tableau)
    let client_y = 398;
    let client_h = 140;

    let mut client_title = Frame::new(20, client_y, win_w - 40, 20, "APPAREILS CONNECTÉS SUR LE RÉSEAU");
    client_title.set_label_color(COLOR_TEXT_WHITE);
    client_title.set_label_font(Font::HelveticaBold);
    client_title.set_label_size(12);
    client_title.set_align(Align::Left | Align::Inside);

    let mut client_browser = HoldBrowser::new(20, client_y + 24, win_w - 40, client_h, None);
    client_browser.set_color(COLOR_CARD);
    client_browser.set_text_size(12);
    client_browser.set_column_char('\t');
    client_browser.set_column_widths(&[200, 260, 160, 0]);
    // En-tête Cyan éclatant (@C223 = FL_CYAN) en gras (@b) monospace (@f)
    client_browser.add("@b@f@C223  ADRESSE IP\t@b@f@C223ADRESSE MAC\t@b@f@C223ÉTAT");

    // 6. CONSOLE DE LOGS / ACTIVITÉ
    let log_y = 574;
    let log_h = 106;

    let mut log_title = Frame::new(20, log_y, win_w - 40, 18, "JOURNAL D'ACTIVITÉ");
    log_title.set_label_color(COLOR_TEXT_MUTED);
    log_title.set_label_font(Font::HelveticaBold);
    log_title.set_label_size(11);
    log_title.set_align(Align::Left | Align::Inside);

    let mut log_browser = HoldBrowser::new(20, log_y + 20, win_w - 40, log_h, None);
    log_browser.set_color(COLOR_INPUT);
    log_browser.set_text_size(11);

    wind.end();
    wind.show();

    // Helper pour log : Préfixe @f@C255 pour forcer le texte BLANC PUR (255 = FL_WHITE)
    let add_log = |browser: &mut HoldBrowser, msg: &str| {
        let now = chrono_now_str();
        browser.add(&format!("@f@C255  [{}] {}", now, msg));
        let count = browser.size();
        if count > 0 {
            browser.bottom_line(count);
        }
    };

    // Détection initiale
    let wifi_iface = network::get_wifi_interface();
    let is_init_active = network::is_hotspot_active();
    let (ssid, psk) = network::get_hotspot_config();

    ssid_input.set_value(&ssid);
    pwd_input.set_value(&psk);
    c1_val.set_label(&wifi_iface);

    if is_init_active {
        status_badge.set_label("● HOTSPOT ACTIF");
        status_badge.set_label_color(COLOR_EMERALD_TEXT);
        c2_val.set_label(&network::get_gateway_ip(&wifi_iface));
        btn_toggle.set_label("■  ARRÊTER LE PARTAGE");
        btn_toggle.set_color(COLOR_RED);
        add_log(&mut log_browser, "Point d'accès Wi-Fi actuellement ACTIF.");
    } else {
        status_badge.set_label("○ HOTSPOT INACTIF");
        status_badge.set_label_color(COLOR_RED_TEXT);
        c2_val.set_label("--");
        btn_toggle.set_label("▶  DÉMARRER LE PARTAGE");
        btn_toggle.set_color(COLOR_EMERALD);
        add_log(&mut log_browser, "Point d'accès Wi-Fi actuellement INACTIF.");
    }

    // Thread de fond pour rafraîchissement ultra-léger (surveillance directe sans surcharge)
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_thread = is_running.clone();
    let thread_sender = sender.clone();
    let thread_iface = wifi_iface.clone();

    thread::spawn(move || {
        while is_running_thread.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(3000));
            let status = network::query_quick_status(&thread_iface);
            thread_sender.send(AppMsg::Tick(status));
        }
    });

    let mut last_active: Option<bool> = Some(is_init_active);
    let mut last_clients: Vec<ConnectedClient> = Vec::new();

    // Boucle d'événements principale
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            let mut need_redraw = false;

            match msg {
                AppMsg::TogglePower => {
                    let currently_active = last_active.unwrap_or(false);
                    if currently_active {
                        // Action d'arrêt
                        add_log(&mut log_browser, "Arrêt du hotspot en cours...");
                        match network::stop_hotspot() {
                            Ok(res) => {
                                add_log(&mut log_browser, &res);
                                status_badge.set_label("○ HOTSPOT INACTIF");
                                status_badge.set_label_color(COLOR_RED_TEXT);
                                c2_val.set_label("--");
                                c3_val.set_label("0");
                                btn_toggle.set_label("▶  DÉMARRER LE PARTAGE");
                                btn_toggle.set_color(COLOR_EMERALD);
                                client_browser.clear();
                                client_browser.add("@b@f@C223  ADRESSE IP\t@b@f@C223ADRESSE MAC\t@b@f@C223ÉTAT");
                                client_browser.add("@f@C255  ○ Le point d'accès est inactif.");
                                last_active = Some(false);
                                last_clients.clear();
                            }
                            Err(e) => {
                                add_log(&mut log_browser, &format!("Erreur: {}", e));
                            }
                        }
                    } else {
                        // Action de démarrage
                        let ssid = ssid_input.value().trim().to_string();
                        let pwd = pwd_input.value().trim().to_string();
                        let iface = c1_val.label();

                        add_log(&mut log_browser, &format!("Démarrage du hotspot (SSID: '{}')...", ssid));
                        match network::start_hotspot(&ssid, &pwd, &iface) {
                            Ok(res) => {
                                add_log(&mut log_browser, &res);
                                status_badge.set_label("● HOTSPOT ACTIF");
                                status_badge.set_label_color(COLOR_EMERALD_TEXT);
                                c2_val.set_label(&network::get_gateway_ip(&iface));
                                btn_toggle.set_label("■  ARRÊTER LE PARTAGE");
                                btn_toggle.set_color(COLOR_RED);
                                last_active = Some(true);
                            }
                            Err(e) => {
                                add_log(&mut log_browser, &format!("Erreur: {}", e));
                            }
                        }
                    }
                    need_redraw = true;
                }
                AppMsg::Apply => {
                    let ssid = ssid_input.value().trim().to_string();
                    let pwd = pwd_input.value().trim().to_string();

                    add_log(&mut log_browser, "Application de la configuration...");
                    match network::update_config(&ssid, &pwd) {
                        Ok(res) => {
                            add_log(&mut log_browser, &res);
                        }
                        Err(e) => {
                            add_log(&mut log_browser, &format!("Erreur: {}", e));
                        }
                    }
                    need_redraw = true;
                }
                AppMsg::ToggleShowPassword(hide) => {
                    if hide {
                        pwd_input.set_type(fltk::input::InputType::Secret);
                    } else {
                        pwd_input.set_type(fltk::input::InputType::Normal);
                    }
                    pwd_input.redraw();
                }
                AppMsg::Tick(status) => {
                    if last_active != Some(status.is_active) {
                        last_active = Some(status.is_active);
                        if status.is_active {
                            status_badge.set_label("● HOTSPOT ACTIF");
                            status_badge.set_label_color(COLOR_EMERALD_TEXT);
                            c2_val.set_label("10.42.0.1");
                            btn_toggle.set_label("■  ARRÊTER LE PARTAGE");
                            btn_toggle.set_color(COLOR_RED);
                        } else {
                            status_badge.set_label("○ HOTSPOT INACTIF");
                            status_badge.set_label_color(COLOR_RED_TEXT);
                            c2_val.set_label("--");
                            btn_toggle.set_label("▶  DÉMARRER LE PARTAGE");
                            btn_toggle.set_color(COLOR_EMERALD);
                        }
                        need_redraw = true;
                    }

                    if last_clients != status.clients {
                        c3_val.set_label(&status.clients.len().to_string());
                        client_browser.clear();
                        client_browser.add("@b@f@C223  ADRESSE IP\t@b@f@C223ADRESSE MAC\t@b@f@C223ÉTAT");
                        if status.clients.is_empty() {
                            if status.is_active {
                                client_browser.add("@f@C255  ○ Aucun appareil connecté pour le moment.");
                            } else {
                                client_browser.add("@f@C255  ○ Le point d'accès est inactif.");
                            }
                        } else {
                            for client in &status.clients {
                                let line = format!(
                                    "@f@C255  {}\t@f@C255{}\t@b@f@C63{}",
                                    client.ip, client.mac, client.state
                                );
                                client_browser.add(&line);
                            }
                        }
                        last_clients = status.clients;
                        need_redraw = true;
                    }
                }
                AppMsg::Log(text) => {
                    add_log(&mut log_browser, &text);
                    need_redraw = true;
                }
            }

            if need_redraw {
                wind.redraw();
            }
        }
    }

    is_running.store(false, Ordering::Relaxed);
    Ok(())
}

fn chrono_now_str() -> String {
    if let Ok(duration) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        let secs = duration.as_secs();
        let s = secs % 60;
        let m = (secs / 60) % 60;
        let h = ((secs / 3600) + 3) % 24; // Fuseau horaire (+03:00)
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        "00:00:00".to_string()
    }
}
