-- Таблица пользователей админ-панели
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) DEFAULT 'manager', -- admin, manager, storekeeper
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Таблица юрлиц (Клиенты)
CREATE TABLE IF NOT EXISTS companies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    inn VARCHAR(20) UNIQUE NOT NULL,
    kpp VARCHAR(20),
    legal_address TEXT,
    contact_person VARCHAR(100),
    phone VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Таблица кофемашин (привязаны к юрлицам и имеют уникальный QR-код)
CREATE TABLE IF NOT EXISTS machines (
    id SERIAL PRIMARY KEY,
    company_id INTEGER REFERENCES companies(id) ON DELETE CASCADE,
    model VARCHAR(100) NOT NULL,
    serial_number VARCHAR(100) UNIQUE NOT NULL,
    installation_address TEXT NOT NULL,
    qr_token VARCHAR(64) UNIQUE NOT NULL, -- Токен для ссылки QR-кода
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Таблица товаров / склада
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    sku VARCHAR(50) UNIQUE NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    stock_quantity INTEGER NOT NULL DEFAULT 0,    -- Реальный остаток на полке
    reserved_quantity INTEGER NOT NULL DEFAULT 0, -- Заморожено под новые заказы
    unit VARCHAR(20) DEFAULT 'шт'                -- шт, кг, л
);

-- Таблица заказов
CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    company_id INTEGER REFERENCES companies(id) ON DELETE SET NULL,
    machine_id INTEGER REFERENCES machines(id) ON DELETE SET NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'NEW',       -- NEW, CONFIRMED, SHIPPED, COMPLETED, CANCELLED
    payment_method VARCHAR(20) NOT NULL DEFAULT 'cash', -- cash, bank_transfer
    payment_status VARCHAR(20) NOT NULL DEFAULT 'unpaid', -- unpaid, paid, overdue
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Позиции в заказе
CREATE TABLE IF NOT EXISTS order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL,
    price DECIMAL(10, 2) NOT NULL
);
