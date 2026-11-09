#!/bin/bash
set -e

echo "=== Установка панели управления кофемашинами «Бариста» ==="

read -p "Введите ваш домен (например, barista.mydomain.com): " DOMAIN
read -p "Введите ваш Email (для SSL Let's Encrypt): " EMAIL

echo "Создание конфигурации (.env)..."
cat <<EOF > .env
DOMAIN=${DOMAIN}
EMAIL=${EMAIL}
POSTGRES_USER=barista
POSTGRES_PASSWORD=$(openssl rand -hex 12)
POSTGRES_DB=baristadb
EOF

echo "Запуск сервисов через Docker Compose..."
if command -v docker-compose &> /dev/null; then
    docker-compose up -d --build
else
    docker compose up -d --build
fi

echo "=== Развертывание успешно завершено! ==="
echo "Панель доступна по адресу: http://${DOM
AIN}"
