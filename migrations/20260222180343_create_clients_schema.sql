-- =============================================================================
-- SCHEMA: Gerenciamento de Projetos de Obras com foco em Pintura
-- Banco de dados: PostgreSQL
-- Convenções:
--   tb_   → tabelas
--   pk_   → primary key (UUID, gerado pela aplicação)
--   fk_   → foreign key
--   idx_  → índice único auto-incrementado (SERIAL)
--   tx_   → colunas texto/varchar
--   nr_   → colunas numéricas (integer, numeric, decimal)
--   dt_   → colunas date
--   ts_   → colunas timestamp
--   bl_   → colunas boolean
-- =============================================================================

-- =============================================================================
-- SCHEMA CLIENTS
-- Agrupa todas as entidades relacionadas a clientes, projetos e alocações.
-- Depende dos schemas: contacts (tb_contact), locations (tb_location)
--                      e collaborators (tb_collaborator).
-- =============================================================================
CREATE SCHEMA IF NOT EXISTS clients;


-- =============================================================================
-- CLIENTE
-- Pessoa física ou jurídica contratante da obra.
-- Um cliente pode ter múltiplos contatos (tb_client_contact),
-- múltiplos endereços (tb_client_address) e múltiplos projetos (tb_project).
-- =============================================================================
CREATE TABLE clients.tb_client
(
    pk_client            UUID                    NOT NULL,
    idx_client           SERIAL                  NOT NULL,
    tx_name              VARCHAR(64)             NOT NULL,
    tx_doc               VARCHAR(23)            NOT NULL,
    tx_status            VARCHAR(32)             NOT NULL,
    ts_client_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_client_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_client PRIMARY KEY (pk_client),
    CONSTRAINT uq_idx_client UNIQUE (idx_client),
    CONSTRAINT uq_tx_doc UNIQUE (tx_doc)
);


-- =============================================================================
-- CONTATO DO CLIENTE
-- Tabela de associação (N:N) entre clientes e contatos.
-- Os contatos são gerenciados no schema externo contacts.tb_contact.
-- As constraints UNIQUE em fk_client e fk_contact modelam relação 1:1
-- (cada cliente tem um contato principal e cada contato pertence a um cliente).
-- Remova os UNIQUE se a regra de negócio permitir múltiplos contatos por cliente.
-- =============================================================================
CREATE TABLE clients.tb_client_contact
(
    pk_client_contact            UUID                    NOT NULL,
    idx_client_contact           SERIAL                  NOT NULL,
    fk_client                    UUID                    NOT NULL,
    fk_contact                   UUID                    NOT NULL,
    ts_client_contact_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_client_contact_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_client_contact PRIMARY KEY (pk_client_contact),
    CONSTRAINT uq_idx_client_contact UNIQUE (idx_client_contact),
    CONSTRAINT uq_fk_client_contact_client UNIQUE (fk_client),
    CONSTRAINT uq_fk_client_contact_contact UNIQUE (fk_contact),
    CONSTRAINT fk_client_contact_client FOREIGN KEY (fk_client)
        REFERENCES clients.tb_client (pk_client)
        ON DELETE CASCADE,
    CONSTRAINT fk_client_contact_contact FOREIGN KEY (fk_contact)
        REFERENCES contacts.tb_contact (pk_contact)
        ON DELETE RESTRICT
);

-- Índices de performance para buscas por cliente e por contato
CREATE INDEX idx_tb_client_contact_fk_client
    ON clients.tb_client_contact (fk_client);
CREATE INDEX idx_tb_client_contact_fk_contact
    ON clients.tb_client_contact (fk_contact);


-- =============================================================================
-- ENDEREÇO DO CLIENTE
-- Tabela de associação entre clientes e localizações físicas.
-- Os endereços são gerenciados no schema externo locations.tb_location,
-- permitindo reuso de endereços entre entidades do sistema.
-- Um cliente pode ter múltiplos endereços (ex: sede, filial, obra).
-- =============================================================================
CREATE TABLE clients.tb_client_address
(
    pk_client_address            UUID                    NOT NULL,
    idx_client_address           SERIAL                  NOT NULL,
    fk_client                    UUID                    NOT NULL,
    fk_address                   UUID                    NOT NULL,
    ts_client_address_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_client_address_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_client_address PRIMARY KEY (pk_client_address),
    CONSTRAINT uq_idx_client_address UNIQUE (idx_client_address),
    CONSTRAINT fk_client_address_client FOREIGN KEY (fk_client)
        REFERENCES clients.tb_client (pk_client)
        ON DELETE CASCADE,
    CONSTRAINT fk_client_address_location FOREIGN KEY (fk_address)
        REFERENCES locations.tb_location (pk_location)
        ON DELETE RESTRICT
);

-- Índices de performance para buscas por cliente e por endereço
CREATE INDEX idx_tb_client_address_fk_client ON clients.tb_client_address (fk_client);
CREATE INDEX idx_tb_client_address_fk_address ON clients.tb_client_address (fk_address);


-- =============================================================================
-- PROJETO DO CLIENTE
-- Tabela de associação (1:N) entre clientes e projetos.
-- Os projetos são gerenciados no schema externo project.tb_project.
-- Um cliente pode ter múltiplos projetos, mas um projeto pertence a
-- apenas um cliente (garantido pela constraint UNIQUE em fk_project).
-- =============================================================================
CREATE TABLE clients.tb_client_project
(
    pk_client_project            UUID                    NOT NULL,
    idx_client_project           SERIAL                  NOT NULL,
    fk_client                    UUID                    NOT NULL,
    fk_project                   UUID                    NOT NULL,
    ts_client_project_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_client_project_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_client_project PRIMARY KEY (pk_client_project),
    CONSTRAINT uq_idx_client_project UNIQUE (idx_client_project),
    -- 1:N: cada projeto pertence a apenas um cliente
    CONSTRAINT uq_fk_client_project UNIQUE (fk_client, fk_project),
    CONSTRAINT fk_client_project_client FOREIGN KEY (fk_client)
        REFERENCES clients.tb_client (pk_client)
        ON DELETE CASCADE,
    CONSTRAINT fk_client_project_project FOREIGN KEY (fk_project)
        REFERENCES project.tb_project (pk_project)
        ON DELETE CASCADE
);

-- Índices de performance para buscas por cliente e por projeto
CREATE INDEX idx_tb_client_project_fk_client
    ON clients.tb_client_project (fk_client);
CREATE INDEX idx_tb_client_project_fk_project
    ON clients.tb_client_project (fk_project);


