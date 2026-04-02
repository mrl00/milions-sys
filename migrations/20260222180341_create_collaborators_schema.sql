-- =============================================================================
-- SCHEMA COLLABORATORS
-- Gerencia os colaboradores de campo (pintores, ajudantes, supervisores)
-- e seus dados de contato e endereço.
-- Depende dos schemas: contacts (tb_contact) e locations (tb_location).
-- =============================================================================
CREATE SCHEMA IF NOT EXISTS collaborators;


-- =============================================================================
-- COLABORADOR
-- Pessoa física que executa serviços nas obras.
-- tx_level indica a especialização: painter | helper | supervisor | generalist.
-- tx_status controla a disponibilidade: active | inactive | suspended.
-- tx_cpf armazenado sem formatação (apenas dígitos): ex: 00000000000.
-- =============================================================================
CREATE TABLE collaborators.tb_collaborator
(
    pk_collaborator            UUID                    NOT NULL,
    idx_collaborator           SERIAL                  NOT NULL,
    tx_name                    VARCHAR(64)             NOT NULL,
    tx_cpf                     VARCHAR(11)             NOT NULL,
    tx_level                   VARCHAR(16)             NOT NULL,
    tx_status                  VARCHAR(16)             NOT NULL,
    ts_collaborator_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_collaborator_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_collaborator PRIMARY KEY (pk_collaborator),
    CONSTRAINT uq_idx_collaborator UNIQUE (idx_collaborator),
    CONSTRAINT uq_tx_cpf UNIQUE (tx_cpf)
);


-- =============================================================================
-- CONTATO DO COLABORADOR
-- Tabela de associação entre colaboradores e contatos.
-- Os contatos (e-mail e telefones) são gerenciados no schema contacts.tb_contact.
-- As constraints UNIQUE em fk_collaborator e fk_contact modelam relação 1:1
-- (cada colaborador tem um contato principal e cada contato pertence a um colaborador).
-- Remova os UNIQUE se a regra de negócio permitir múltiplos contatos por colaborador.
-- =============================================================================
CREATE TABLE collaborators.tb_collaborator_contact
(
    pk_collaborator_contact            UUID                    NOT NULL,
    idx_collaborator_contact           SERIAL                  NOT NULL,
    fk_collaborator                    UUID                    NOT NULL,
    fk_contact                         UUID                    NOT NULL,
    ts_collaborator_contact_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_collaborator_contact_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_collaborator_contact PRIMARY KEY (pk_collaborator_contact),
    CONSTRAINT uq_idx_collaborator_contact UNIQUE (idx_collaborator_contact),
    CONSTRAINT uq_fk_collaborator_contact_collaborator UNIQUE (fk_collaborator),
    CONSTRAINT uq_fk_collaborator_contact_contact UNIQUE (fk_contact),
    CONSTRAINT fk_collaborator_contact_collaborator FOREIGN KEY (fk_collaborator)
        REFERENCES collaborators.tb_collaborator (pk_collaborator)
        ON DELETE CASCADE,
    CONSTRAINT fk_collaborator_contact_contact FOREIGN KEY (fk_contact)
        REFERENCES contacts.tb_contact (pk_contact)
        ON DELETE RESTRICT
);

-- Índices de performance para buscas por colaborador e por contato
CREATE INDEX idx_tb_collaborator_contact_fk_collaborator
    ON collaborators.tb_collaborator_contact (fk_collaborator);
CREATE INDEX idx_tb_collaborator_contact_fk_contact
    ON collaborators.tb_collaborator_contact (fk_contact);


-- =============================================================================
-- ENDEREÇO DO COLABORADOR
-- Tabela de associação entre colaboradores e localizações físicas.
-- Os endereços são gerenciados no schema externo locations.tb_location,
-- permitindo reuso de endereços entre entidades do sistema.
-- Um colaborador pode ter múltiplos endereços (ex: residencial, correspondência).
-- =============================================================================
CREATE TABLE collaborators.tb_collaborator_address
(
    pk_collaborator_address            UUID                    NOT NULL,
    idx_collaborator_address           SERIAL                  NOT NULL,
    fk_collaborator                    UUID                    NOT NULL,
    fk_address                         UUID                    NOT NULL,
    ts_collaborator_address_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_collaborator_address_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_collaborator_address PRIMARY KEY (pk_collaborator_address),
    CONSTRAINT uq_idx_collaborator_address UNIQUE (idx_collaborator_address),
    CONSTRAINT fk_collaborator_address_collaborator FOREIGN KEY (fk_collaborator)
        REFERENCES collaborators.tb_collaborator (pk_collaborator)
        ON DELETE CASCADE,
    CONSTRAINT fk_collaborator_address_location FOREIGN KEY (fk_address)
        REFERENCES locations.tb_location (pk_location)
        ON DELETE RESTRICT
);

-- Índices de performance para buscas por colaborador e por endereço
CREATE INDEX idx_tb_collaborator_address_fk_collaborator
    ON collaborators.tb_collaborator_address (fk_collaborator);
CREATE INDEX idx_tb_collaborator_address_fk_address
    ON collaborators.tb_collaborator_address (fk_address);