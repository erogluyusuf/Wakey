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

    // Yıldızlı yolu (*) bash üzerinden bulan garantici fonksiyon
    fn set_kbd_brightness(val: &str) {
        let cmd = format!("echo {} | sudo tee /sys/class/leds/*kbd_backlight/brightness", val);
        let _ = Command::new("sh").arg("-c").arg(&cmd).output();
    }

    fn set_tuxedo_fan(profile: &str) {
        let _ = Command::new("busctl")
            .args(&["call", "com.tuxedocomputers.tccd", "/com/tuxedocomputers/tccd", "com.tuxedocomputers.tccd", "SetTempProfileById", "s", profile])
            .output();
    }

    // CPU Tasarruf Modu (Uygulamaları dondurmadan pil koruma)
    fn set_cpu_epp(mode: &str) {
        // mode: "power" veya "balance_performance"
        let cmd = format!("echo {} | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference", mode);
        let _ = Command::new("sh").arg("-c").arg(&cmd).output();
    }

    pub async fn start_monitoring() -> anyhow::Result<()> {
        info!("💡 Monitor: Stabilite Odaklı Tasarruf Modu");

        let lid_path = match Self::find_lid_path() {
            Some(p) => p,
            None => {
                error!("❌ Kapak sensörü yok!");
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
                            info!("🌙 KAPAK KAPANDI: Karanlık ve Sessiz Mod");
                            
                            // 1. Ekran ve Klavye Işığını Kapat
                            let _ = Command::new("brightnessctl").args(&["set", "0"]).output();
                            Self::set_kbd_brightness("0");
                            
                            // 2. Fan Sessiz ve İşlemci Güç Tasarrufu
                            Self::set_tuxedo_fan("__legacy_default__");
                            Self::set_cpu_epp("power");

                        } else {
                            info!("☀️ KAPAK AÇILDI: Tam Performans");

                            // 1. Donanımı Uyandır
                            Self::set_tuxedo_fan("Default");
                            Self::set_cpu_epp("balance_performance");
                            let _ = Command::new("brightnessctl").args(&["set", "100%"]).output();
                            
                            // 2. Klavyeyi %80 (200) Aç
                            Self::set_kbd_brightness("200");
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
