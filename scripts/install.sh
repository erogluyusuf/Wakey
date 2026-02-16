#!/bin/bash

# Renk Kodları
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}==============================================${NC}"
echo -e "${BLUE}   ⚡ Wakey - Kurulum Devam Ediyor...         ${NC}"
echo -e "${BLUE}==============================================${NC}"

if [ "$EUID" -ne 0 ]; then
  echo -e "${RED}❌ Lütfen 'sudo' ile çalıştırın.${NC}"
  exit 1
fi

# 1. Ayar Kontrolü (Zaten yapılmış olmalı ama garantiye alalım)
echo -e "\n${YELLOW}[1/5] Ayar Dosyası Kontrolü...${NC}"
CONF_FILE="/etc/systemd/logind.conf.d/99-wakey.conf"
# Dosyayı tekrar yazalım ama servisi restart ETMEYELİM (Oturum gitmesin diye)
mkdir -p "/etc/systemd/logind.conf.d"
echo -e "[Login]\nHandleLidSwitch=ignore\nHandleLidSwitchExternalPower=ignore\nHandleLidSwitchDocked=ignore" > "$CONF_FILE"
echo -e "${GREEN}✅ Ayar dosyası doğrulandı.${NC}"


# 2. Paketler
echo -e "\n${YELLOW}[2/5] Paketler Kontrol Ediliyor...${NC}"
if ! command -v brightnessctl &> /dev/null; then
    dnf install -y brightnessctl
else
    echo -e "${GREEN}✅ brightnessctl hazır.${NC}"
fi


# 3. Donanım Analizi
echo -e "\n${YELLOW}[3/5] Donanım Taraması...${NC}"
busctl list | grep -q "com.tuxedocomputers.tccd" && echo -e "${GREEN}✅ TUXEDO TESPİT EDİLDİ.${NC}"


# 4. Derleme (Compile)
echo -e "\n${YELLOW}[4/5] Wakey Derleniyor...${NC}"
# Root ortamında cargo yolunu bulamazsa diye environment sourced
if [ -f "$HOME/.cargo/env" ]; then source "$HOME/.cargo/env"; fi

cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Derleme hatası! Lütfen script bitince elle 'cargo build --release' yapın.${NC}"
    exit 1
fi


# 5. Kurulum ve Servis
echo -e "\n${YELLOW}[5/5] Servis Kuruluyor...${NC}"
cp target/release/wakey /usr/local/bin/wakey
chmod +x /usr/local/bin/wakey

cat <<SERVICE > /etc/systemd/system/wakey.service
[Unit]
Description=Wakey Instant-On Daemon
After=network.target dbus.service systemd-logind.service

[Service]
Type=simple
ExecStart=/usr/local/bin/wakey
Restart=always
User=root
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
SERVICE

systemctl daemon-reload
systemctl enable wakey
systemctl restart wakey

echo -e "\n${GREEN}🎉 KURULUM BAŞARIYLA TAMAMLANDI! ${NC}"
echo -e "Wakey şu an arka planda çalışıyor."
