-- =============================================================================
-- SCHEMA CONTACTS
-- Centraliza dados de contato reutilizáveis por qualquer entidade do sistema
-- (clientes, colaboradores, fornecedores etc.).
-- Um contato agrupa e-mail e múltiplos telefones sob um mesmo registro,
-- permitindo que outras entidades referenciem contatos via FK sem duplicar dados.
-- =============================================================================
CREATE SCHEMA IF NOT EXISTS contacts;


-- =============================================================================
-- CONTATO
-- Registro raiz de contato, identificado principalmente pelo e-mail.
-- Telefones associados são armazenados em tb_phone (relação 1:N).
-- tx_email é nullable para casos onde o contato só possui telefone.
-- =============================================================================
CREATE TABLE contacts.tb_contact
(
    pk_contact            UUID                    NOT NULL,
    idx_contact           SERIAL                  NOT NULL,
    tx_email              VARCHAR(256),
    ts_contact_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_contact_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_contact       PRIMARY KEY (pk_contact),
    CONSTRAINT uq_idx_contact   UNIQUE (idx_contact),
    CONSTRAINT uq_tx_email      UNIQUE (tx_email)
);


-- =============================================================================
-- TELEFONE
-- Telefones vinculados a um contato (celular, fixo, WhatsApp etc.).
-- Um contato pode ter múltiplos telefones, mas cada número é único no sistema.
-- tx_phone deve ser armazenado com DDD e sem formatação: ex: 61999990000.
-- =============================================================================
CREATE TABLE contacts.tb_phone
(
    pk_phone            UUID                    NOT NULL,
    idx_phone           SERIAL                  NOT NULL,
    fk_contact          UUID                    NOT NULL,
    tx_phone            VARCHAR(16)             NOT NULL,
    ts_phone_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_phone_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_phone         PRIMARY KEY (pk_phone),
    CONSTRAINT uq_idx_phone     UNIQUE (idx_phone),
    CONSTRAINT uq_tx_phone      UNIQUE (tx_phone),
    CONSTRAINT fk_phone_contact FOREIGN KEY (fk_contact)
        REFERENCES contacts.tb_contact (pk_contact)
        ON DELETE CASCADE
);

-- Índice de performance para buscas de todos os telefones de um contato
CREATE INDEX idx_tb_phone_fk_contact ON contacts.tb_phone (fk_contact);