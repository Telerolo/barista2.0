#!/bin/bash
set -e

echo "=== Установка панели управления кофемашинами «Бариста» ==="

# Проверка наличия Docker
if ! command -v docker &> /dev/null; then
    echo "Docker не найден. Устанавливаем Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
    rm get-docker.sh
    echo "Docker успешно установлен."
fi

# Запрос данных у пользователя
read -p "Введите ваш домен (например, barista.mydomain.com): " DOMAIN
read -p "Введите ваш Email (для SSL Let's Encrypt): " EMAIL

if [ -z "$DOMAIN" ] || [ -z "$EMAIL" ]; then
    echo "Ошибка: Домен и Email обязательны!"
    exit 1
fi

# Создание конфигурации .env
echo "Создание конфигурации (.env)..."
cat << EOF > .env
DOMAIN=$DOMAIN
SSL_EMAIL=$EMAIL
POSTGRES_DB=barista_db
POSTGRES_USER=barista_user
POSTGRES_PASSWORD=$(openssl rand -hex 16)
DATABASE_URL=postgres://barista_user:$(openssl rand -hex 16)@db:5432/barista_db
JWT_SECRET=$(openssl rand -hex 32)
EOF

# Создаем нужные папки
mkdir -p nginx/conf.d backend/src frontend/src docs certbot/conf certbot/www

# Запуск контейнеров
echo "Запуск сервисов через Docker Compose..."
docker compose down --remove-orphans
docker compose up -d --build

echo "=== Установка завершена! ==="
echo "Панель будет доступна по адресу: https://$DOMAIN после выпуска SSL."
