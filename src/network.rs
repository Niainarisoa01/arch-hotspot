use anyhow::{anyhow, Result};
use std::fs;
use std::process::Command;

pub const HOTSPOT_CON_NAME: &str = "WifiPartage";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectedClient {
    pub ip: String,
    pub mac: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct QuickTickStatus {
    pub is_active: bool,
    pub clients: Vec<ConnectedClient>,
}

/// Détecte la première interface Wi-Fi disponible sur le système
pub fn get_wifi_interface() -> String {
    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.join("wireless").exists() || path.join("phy80211").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    return name.to_string();
                }
            }
        }
    }
    // Secours via nmcli si sysfs est différent
    let output = Command::new("nmcli")
        .args(["-t", "-f", "DEVICE,TYPE", "device"])
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && parts[1] == "wifi" {
                return parts[0].to_string();
            }
        }
    }
    "wlp4s0".to_string()
}

/// Vérifie si la connexion Hotspot est actuellement active
pub fn is_hotspot_active() -> bool {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "NAME,TYPE", "connection", "show", "--active"])
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if !parts.is_empty() && parts[0] == HOTSPOT_CON_NAME {
                return true;
            }
        }
    }
    false
}

/// Récupère la configuration actuelle du hotspot (SSID et mot de passe PSK)
pub fn get_hotspot_config() -> (String, String) {
    let output = Command::new("nmcli")
        .args([
            "-s",
            "-g",
            "802-11-wireless.ssid,802-11-wireless-security.psk",
            "connection",
            "show",
            HOTSPOT_CON_NAME,
        ])
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        let lines: Vec<&str> = text.lines().collect();
        let ssid = lines.first().unwrap_or(&"Niaina").trim().to_string();
        let psk = lines.get(1).unwrap_or(&"12345678").trim().to_string();
        (
            if ssid.is_empty() { "Niaina".to_string() } else { ssid },
            if psk.is_empty() { "12345678".to_string() } else { psk },
        )
    } else {
        ("Niaina".to_string(), "12345678".to_string())
    }
}

/// Récupère l'adresse IP de passerelle assignée au hotspot (ex: 10.42.0.1)
pub fn get_gateway_ip(interface: &str) -> String {
    let output = Command::new("ip")
        .args(["-4", "-o", "addr", "show", "dev", interface])
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for part in text.split_whitespace() {
            if part.contains('/') && (part.starts_with("10.") || part.starts_with("192.168.")) {
                if let Some(ip) = part.split('/').next() {
                    return ip.to_string();
                }
            }
        }
    }
    "10.42.0.1".to_string()
}

/// Récupère les clients connectés via lecture directe de /proc/net/arp (Zéro sous-processus)
pub fn get_connected_clients(interface: &str) -> Vec<ConnectedClient> {
    let mut clients = Vec::new();

    // Lecture directe dans le noyau Linux : 0 fork, 0 exec, exécution en microsecondes
    if let Ok(content) = fs::read_to_string("/proc/net/arp") {
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Format: IP, HW_type, Flags, HW_addr, Mask, Device
            if parts.len() >= 6 {
                let ip = parts[0];
                let flags = parts[2];
                let mac = parts[3];
                let dev = parts[5];

                // Flags "0x0" signifie incomplète / déconnectée
                if dev == interface && flags != "0x0" && mac != "00:00:00:00:00:00" {
                    clients.push(ConnectedClient {
                        ip: ip.to_string(),
                        mac: mac.to_uppercase(),
                        state: "CONNECTÉ".to_string(),
                    });
                }
            }
        }
    }

    clients
}

/// Surveillance ultra-rapide et économe (Tick périodique)
pub fn query_quick_status(interface: &str) -> QuickTickStatus {
    let active = is_hotspot_active();
    let clients = if active {
        get_connected_clients(interface)
    } else {
        Vec::new()
    };

    QuickTickStatus {
        is_active: active,
        clients,
    }
}

/// Démarre ou réactive le point d'accès Wi-Fi
pub fn start_hotspot(ssid: &str, password: &str, iface: &str) -> Result<String> {
    if password.len() < 8 {
        return Err(anyhow!("Le mot de passe doit contenir au moins 8 caractères"));
    }

    let check = Command::new("nmcli")
        .args(["connection", "show", HOTSPOT_CON_NAME])
        .output()?;

    if check.status.success() {
        let mod_res = Command::new("nmcli")
            .args([
                "connection",
                "modify",
                HOTSPOT_CON_NAME,
                "802-11-wireless.ssid",
                ssid,
                "802-11-wireless-security.psk",
                password,
            ])
            .output()?;

        if !mod_res.status.success() {
            return Err(anyhow!(
                "Erreur configuration nmcli: {}",
                String::from_utf8_lossy(&mod_res.stderr)
            ));
        }

        let up_res = Command::new("nmcli")
            .args(["connection", "up", HOTSPOT_CON_NAME])
            .output()?;

        if up_res.status.success() {
            Ok(format!("Hotspot activé avec succès (SSID: '{}')", ssid))
        } else {
            Err(anyhow!(
                "Erreur activation nmcli: {}",
                String::from_utf8_lossy(&up_res.stderr)
            ))
        }
    } else {
        let create_res = Command::new("nmcli")
            .args([
                "device",
                "wifi",
                "hotspot",
                "ifname",
                iface,
                "con-name",
                HOTSPOT_CON_NAME,
                "ssid",
                ssid,
                "password",
                password,
            ])
            .output()?;

        if create_res.status.success() {
            Ok(format!("Hotspot créé et démarré avec succès (SSID: '{}')", ssid))
        } else {
            Err(anyhow!(
                "Erreur création nmcli: {}",
                String::from_utf8_lossy(&create_res.stderr)
            ))
        }
    }
}

/// Arrête le point d'accès Wi-Fi
pub fn stop_hotspot() -> Result<String> {
    let output = Command::new("nmcli")
        .args(["connection", "down", HOTSPOT_CON_NAME])
        .output()?;

    if output.status.success() {
        Ok("Hotspot désactivé avec succès".to_string())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        if err_msg.contains("aucune connexion active") || err_msg.contains("no active connection") {
            Ok("Le hotspot était déjà inactif".to_string())
        } else {
            Err(anyhow!("Erreur arrêt hotspot: {}", err_msg))
        }
    }
}

/// Met à jour la configuration (SSID & mot de passe)
pub fn update_config(ssid: &str, password: &str) -> Result<String> {
    if password.len() < 8 {
        return Err(anyhow!("Le mot de passe doit contenir au moins 8 caractères"));
    }

    let mod_res = Command::new("nmcli")
        .args([
            "connection",
            "modify",
            HOTSPOT_CON_NAME,
            "802-11-wireless.ssid",
            ssid,
            "802-11-wireless-security.psk",
            password,
        ])
        .output()?;

    if !mod_res.status.success() {
        return Err(anyhow!(
            "Erreur mise à jour nmcli: {}",
            String::from_utf8_lossy(&mod_res.stderr)
        ));
    }

    if is_hotspot_active() {
        let _ = Command::new("nmcli")
            .args(["connection", "up", HOTSPOT_CON_NAME])
            .output();
        Ok(format!("Configuration appliquée et hotspot redémarré (SSID: '{}')", ssid))
    } else {
        Ok(format!("Configuration enregistrée pour le prochain démarrage (SSID: '{}')", ssid))
    }
}
