-- Criação do tipo ENUM para user_type
CREATE TYPE USER_TYPE AS ENUM ('logista', 'comum');

-- Criação da tabela users
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    cpf CHAR(11) UNIQUE,
    cnpj CHAR(14) UNIQUE,
    email VARCHAR(255) UNIQUE NOT NULL,
    senha VARCHAR(255) NOT NULL,
    user_type USER_TYPE NOT NULL
);

-- Criação do tipo ENUM para currency
CREATE TYPE CURRENCY AS ENUM ('BRL', 'USD', 'EUR', 'CNY', 'JPY');

-- Criação da tabela wallets
CREATE TABLE wallets (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    amount FLOAT DEFAULT 0.0,
    currency CURRENCY NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    UNIQUE (user_id, currency),
    CONSTRAINT amount_nonnegative CHECK (amount >= 0)
);

CREATE INDEX currency_index ON wallets USING HASH (currency);

-- Criação do tipo ENUM para status das transfers
CREATE TYPE TRANSFER_STATUS AS ENUM ('pending', 'canceled', 'concluded', 'failed');

-- Criação da tabela transfers
CREATE TABLE transfers (
    id SERIAL PRIMARY KEY,
    payer_id INT NOT NULL,
    payee_id INT NOT NULL,
    amount FLOAT NOT NULL,
    wallet_payer_id INT NOT NULL,
    wallet_payee_id INT NOT NULL,
    currency CURRENCY NOT NULL,
    status TRANSFER_STATUS NOT NULL,
    FOREIGN KEY (payer_id) REFERENCES users(id),
    FOREIGN KEY (payee_id) REFERENCES users(id),
    FOREIGN KEY (wallet_payer_id) REFERENCES wallets(id),
    FOREIGN KEY (wallet_payee_id) REFERENCES wallets(id),
    CONSTRAINT amount_nonnegative CHECK (amount >=0 ),
    CONSTRAINT diferent_user_id CHECK (payer_id != payee_id)
);
