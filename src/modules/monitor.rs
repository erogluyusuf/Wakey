use std::fs;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn, error};
use std::process::Command;
use std::path::Path;

pub struct Monitor;

impl Monitor {
    fn find_lid_path() -> Option<String> {
        let candidates = ["/proc/acpi/button/lid/LID1/state", "/proc/acpi/button/lid/LID0/state"];
        for path in candidates {
            if Path::new(path).exists() { return Some(path.to_string()); }
        }
        None
    }

    pub async fn start_monitoring() -> anyhow::Result<()> {
        info!("💡 Monitor: Saf Donanım Modu Başlatıldı");

        let lid_path = match Self::find_lid_path() {
            Some(p) => p,
            None => {
                error!("❌ Kapak sensörü bulunamadı!");
                return Ok(());
            }
        };
        
        let mut last_state = "open".to_string();

        loop {
            match fs::read_to_string(&lid_path) {
                Ok(content) => {
                    let current_state = if content.contains("closed") { "closed" } else { "open" };

                    if current_state != last_state {
                        if current_state == "closed" {
                            info!("🌙 Kapak Kapandı: SIRALI KAPATMA BAŞLIYOR");
                            
                            // 1. İLK İŞ: KLAVYE KAPAT (Senin yöntem)
                            let _ = Command::new("sh").arg("-c")
                                .arg("echo 0 | sudo tee /sys/class/leds/*kbd_backlight/brightness").output();

                            // 2. FAN SESSİZ
                            let _ = Command::new("busctl")
                                .args(&["call", "com.tuxedocomputers.tccd", "/com/tuxedocomputers/tccd", "com.tuxedocomputers.tccd", "SetTempProfileById", "s", "__legacy_default__"])
                                .output();

                            // 3. MOUSE IŞIĞI (USB AUTO-SUSPEND)
                            let _ = Command::new("sh").arg("-c")
                                .arg("echo auto | sudo tee /sys/bus/usb/devices/*/power/control").output();

                            // 4. EKRAN KARART
                            let _ = Command::new("brightnessctl").args(&["set", "0"]).output();

                        } else {
                            info!("☀️ Kapak Açıldı: SIRALI UYANDIRMA");

                            // 1. EKRAN VE FAN (Sistem canlansın)
                            let _ = Command::new("brightnessctl").args(&["set", "100%"]).output();
                            let _ = Command::new("busctl")
                                .args(&["call", "com.tuxedocomputers.tccd", "/com/tuxedocomputers/tccd", "com.tuxedocomputers.tccd", "SetTempProfileById", "s", "Default"])
                                .output();

                            // 2. KLAVYE %80
                            let _ = Command::new("sh").arg("-c")
                                .arg("echo 200 | sudo tee /sys/class/leds/*kbd_backlight/brightness").output();

                            // 3. MOUSE AKTİF
                            let _ = Command::new("sh").arg("-c")
                                .arg("echo on | sudo tee /sys/bus/usb/devices/*/power/control").output();
                        }
                        last_state = current_state.to_string();
                    }
                }
                Err(e) => warn!("Okuma hatası: {}", e),
            }
            time::sleep(Duration::from_millis(100)).await;
        }
    }
}