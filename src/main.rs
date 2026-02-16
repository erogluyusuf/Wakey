mod modules;

use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use crate::modules::monitor::Monitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Loglama sistemini başlat
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("⚡ Wakey v0.1.0 Başlatılıyor...");
    info!("📂 Hedef: MacOS benzeri uyanma hızı");

    // 2. Monitor servisini başlat (Hata olursa program dursun)
    if let Err(e) = Monitor::start_monitoring().await {
        error!("Monitor başlatılamadı: {}", e);
        error!("İpucu: Bu programı 'sudo' ile çalıştırman gerekebilir (D-Bus erişimi için).");
    }

    Ok(())
}