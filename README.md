# Система управления складом и кофемашинами «Бариста»

B2B-платформа для поставщика кофе и расходников. Включает админ-панель, складской учет с 2-этапным резервированием, привязку кофемашин через QR-коды, генерацию PDF-документов и быстрый запуск одной командой.

---

## 🚀 Архитектура и Стек

- **Backend:** Rust (Axum + SQLx + Tokio) — высокая производительность, надежная работа со складом и генерация QR/PDF.
- **Frontend:** Vue 3 (Vite + Tailwind / CSS-модули) — SPA с двухуровневой навигацией.
- **Database:** PostgreSQL (транзакции, строгий контроль резервов и остатков).
- **Infrastructure:** Docker, Docker Compose, Nginx (Reverse Proxy), Certbot (Let's Encrypt SSL).

### UI/UX Стандарты Интерфейса
- **Layout:** Двухколоночный. Левая часть (`Sidebar`) статична на 100% высоты экрана (`overflow-hidden`), без глобального скролла.
- **Scroll Region:** Независимый вертикальный скролл только внутри правой контентной области.
- **Header:** Заголовок текущего раздела строго по центру правой части шапки.
- **Navigation:** Кнопка «Назад» расположена слева от центрированного заголовка при переходе в подразделы.
- **Multiprofile:** Переключатель профиля/аккаунта размещен в самом низу левого сайдбара.

---

## 📁 Структура Репозитория

```text
barista-admin/
├── README.md                      # Главный манифест архитектуры и Roadmap
├── ARCHITECTURE.md                # Схема базы данных, REST API и бизнес-правила
├── PROJECT_STATE.md               # Актуальный статус разработки (для ИИ-контекста)
├── deploy.sh                      # Скрипт интерактивного авто-деплоя (Docker + SSL)
├── docker-compose.yml             # Контейнеризация (db, backend, nginx, certbot)
├── docs/                          # Подробные отчеты о каждом выполненном шаге
│   ├── step-0-infrastructure.md
│   ├── step-1-warehouse.md
│   ├── step-2-clients-qr.md
│   ├── step-3-orders.md
│   ├── step-4-settings.md
│   └── step-5-public-qr.md
├── backend/                       # Бэкенд на Rust (Axum)
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs                # Инициализация Axum сервера и маршрутов
│       ├── config.rs              # Считывание env-переменных
│       ├── db.rs                  # Пул подключений PostgreSQL (SQLx)
│       ├── models/                # Структуры БД (Warehouse, Client, Order)
│       ├── handlers/              # Эндпоинты (Warehouse, Clients, Orders, Auth)
│       └── services/              # Бизнес-логика (Резервирование, QR, PDF)
├── frontend/                      # Админ-панель на Vue 3
│   ├── package.json
│   ├── Dockerfile
│   ├── vite.config.js
│   └── src/
│       ├── main.js
│       ├── App.vue                # Главный сетчатый Layout (Sidebar + Content Area)
│       ├── router/
│       └── views/                 # Разделы: Orders, Clients, Warehouse, Settings
└── nginx/                         # Конфигурации проксирования и SSL
    └── conf.d/
        └── default.conf
