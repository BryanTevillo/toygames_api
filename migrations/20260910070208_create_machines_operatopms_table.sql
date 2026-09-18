
CREATE TYPE operation_status AS ENUM ('completed', 'pending_audit');
CREATE TYPE operation_type AS ENUM ('refill', 'cut');

CREATE TABLE machine_operations (
    id UUID PRIMARY KEY,
    machine_id UUID NOT NULL REFERENCES machines(id) ON DELETE CASCADE,
    operation_type operation_type NOT NULL,
    status operation_status NOT NULL DEFAULT 'completed',

    -- Fechas clave del proceso
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- Cuándo ocurrió en la calle (siempre va)
    audited_at TIMESTAMPTZ,                          -- Cuándo se auditó en sucursal (NULL si está pendiente)
    
    -- 1. Lo que SÍ o SÍ se registra en la calle (Obligatorio)
    system_coins INT NOT NULL,
    system_prize_out INT NOT NULL,
    lcd_img_url TEXT NOT NULL,
    physical_toys_count SMALLINT NOT NULL,
    toys_added SMALLINT NOT NULL,
    
    -- 2. Lo que SE QUEDA VACÍO (NULL) en la calle y se llena en sucursal (Opcionales)
    client_payout REAL, -- Este se calcula al momento para pagarle al socio
    cash_withdrawn REAL,       -- NULL si está pendiente de auditar
    company_profit REAL,       -- NULL si está pendiente de auditar
    cash_discrepancy REAL     -- NULL si está pendiente de auditar
);