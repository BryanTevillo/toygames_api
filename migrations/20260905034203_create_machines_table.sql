-- Add migration script here
CREATE TYPE machine_status AS ENUM ('work', 'damage', 'warehouse');
CREATE TYPE claw_size AS ENUM ('small', 'medium', 'large');

CREATE TABLE machines (
    id UUID PRIMARY KEY,
    model VARCHAR(255) NOT NULL,
    claw_size claw_size NOT NULL,
    capacity_toys SMALLINT NOT NULL,
    cost_machine REAL NOT NULL,
    status_machine machine_status NOT NULL,
    coin_acceptors SMALLINT NOT NULL,
    bill_acceptor SMALLINT NOT NULL,
    plays_per_plush SMALLINT,
    credit_cost_per_play SMALLINT,
    img_url TEXT,
    purchase_date DATE
);