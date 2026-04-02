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
-- PROJETO
-- Obra contratada por um cliente, com localização física própria.
-- Centraliza todas as informações financeiras e operacionais da obra:
-- etapas (tb_project_stage), serviços contratados (tb_project_service)
-- e alocação diária de colaboradores (tb_project_daily_allocation).
-- nr_actual_cost deve ser atualizado pela aplicação conforme despesas são lançadas.
-- =============================================================================
CREATE TABLE clients.tb_project
(
    pk_project            UUID         NOT NULL,
    idx_project           SERIAL       NOT NULL,
    tx_name               VARCHAR(255) NOT NULL,
    tx_description        TEXT,
    tx_status             VARCHAR(30)  NOT NULL DEFAULT 'planning',
    dt_start_date         DATE,
    dt_estimated_end_date DATE,
    dt_actual_end_date    DATE,
    nr_total_area_m2      NUMERIC(10, 2), -- área total prevista em m²
    nr_estimated_cost     NUMERIC(14, 2),
    nr_actual_cost        NUMERIC(14, 2),
    tx_notes              TEXT,
    bl_active             BOOLEAN      NOT NULL DEFAULT TRUE,
    ts_project_created_at TIMESTAMP    NOT NULL DEFAULT NOW(),
    ts_project_updated_at TIMESTAMP    NOT NULL DEFAULT NOW(),
    fk_client             UUID         NOT NULL,
    fk_address            UUID         NOT NULL,

    CONSTRAINT pk_project PRIMARY KEY (pk_project),
    CONSTRAINT uq_idx_project UNIQUE (idx_project),
    CONSTRAINT ck_project_status CHECK (tx_status IN ('planning', 'in_progress', 'paused', 'completed', 'cancelled')),
    CONSTRAINT fk_project_client FOREIGN KEY (fk_client)
        REFERENCES clients.tb_client (pk_client)
        ON DELETE CASCADE,
    CONSTRAINT fk_project_address FOREIGN KEY (fk_address)
        REFERENCES locations.tb_location (pk_location)
        ON DELETE RESTRICT
);

-- Índices de performance para buscas por cliente e por endereço da obra
CREATE INDEX idx_tb_project_fk_client ON clients.tb_project (fk_client);
CREATE INDEX idx_tb_project_fk_address ON clients.tb_project (fk_address);


-- =============================================================================
-- ETAPA DO PROJETO
-- Fases de execução: Preparação, Selagem, Massa Corrida, Pintura Base, Acabamento
-- =============================================================================
CREATE TABLE clients.tb_project_stage
(
    pk_project_stage            UUID         NOT NULL,
    idx_project_stage           SERIAL       NOT NULL,
    fk_project                  UUID         NOT NULL,
    tx_name                     VARCHAR(255) NOT NULL,
    tx_description              TEXT,
    nr_order                    INTEGER      NOT NULL DEFAULT 1,
    tx_status                   VARCHAR(30)  NOT NULL DEFAULT 'pending',
    dt_start_date               DATE,
    dt_end_date                 DATE,
    ts_created_at_project_stage TIMESTAMP    NOT NULL DEFAULT NOW(),
    ts_updated_at_project_stage TIMESTAMP    NOT NULL DEFAULT NOW(),

    CONSTRAINT pk_project_stage PRIMARY KEY (pk_project_stage),
    CONSTRAINT uq_idx_project_stage UNIQUE (idx_project_stage),
    CONSTRAINT ck_project_stage_status CHECK (tx_status IN ('pending', 'in_progress', 'completed', 'skipped')),
    CONSTRAINT fk_project_stage_project FOREIGN KEY (fk_project)
        REFERENCES clients.tb_project (pk_project)
        ON DELETE CASCADE
);


-- =============================================================================
-- TIPO DE SERVIÇO
-- Catálogo de serviços oferecidos (Pintura Interna, Textura, Massa Corrida etc.)
-- =============================================================================
CREATE TABLE clients.tb_service_type
(
    pk_service_type            UUID         NOT NULL,
    idx_service_type           SERIAL       NOT NULL,
    tx_name                    VARCHAR(255) NOT NULL, -- ex: Pintura Interna Lisa
    tx_description             TEXT,
    tx_unit                    VARCHAR(20)  NOT NULL DEFAULT 'm2',
    nr_default_unit_price      NUMERIC(10, 2),
    bl_active                  BOOLEAN      NOT NULL DEFAULT TRUE,
    ts_created_at_service_type TIMESTAMP    NOT NULL DEFAULT NOW(),
    ts_updated_at_service_type TIMESTAMP    NOT NULL DEFAULT NOW(),

    CONSTRAINT pk_service_type PRIMARY KEY (pk_service_type),
    CONSTRAINT uq_idx_service_type UNIQUE (idx_service_type),
    CONSTRAINT ck_service_type_unit CHECK (tx_unit IN ('m2', 'm_linear', 'unit', 'hour'))
);


-- =============================================================================
-- SERVIÇO DO PROJETO
-- Serviços contratados dentro de um projeto (com quantitativo e preço)
-- =============================================================================
CREATE TABLE clients.tb_project_service
(
    pk_project_service            UUID           NOT NULL,
    idx_project_service           SERIAL         NOT NULL,
    fk_project                    UUID           NOT NULL,
    fk_project_stage              UUID, -- etapa vinculada (opcional)
    fk_service_type               UUID           NOT NULL,
    tx_description                TEXT,
    nr_quantity                   NUMERIC(10, 2) NOT NULL,
    nr_unit_price                 NUMERIC(10, 2) NOT NULL,
    nr_total_price                NUMERIC(14, 2) GENERATED ALWAYS AS (nr_quantity * nr_unit_price) STORED,
    tx_status                     VARCHAR(30)    NOT NULL DEFAULT 'pending',
    ts_created_at_project_service TIMESTAMP      NOT NULL DEFAULT NOW(),
    ts_updated_at_project_service TIMESTAMP      NOT NULL DEFAULT NOW(),

    CONSTRAINT pk_project_service PRIMARY KEY (pk_project_service),
    CONSTRAINT uq_idx_project_service UNIQUE (idx_project_service),
    CONSTRAINT ck_project_service_status CHECK (tx_status IN ('pending', 'in_progress', 'completed')),
    CONSTRAINT fk_project_service_project FOREIGN KEY (fk_project)
        REFERENCES clients.tb_project (pk_project)
        ON DELETE CASCADE,
    CONSTRAINT fk_project_service_stage FOREIGN KEY (fk_project_stage)
        REFERENCES clients.tb_project_stage (pk_project_stage)
        ON DELETE SET NULL,
    CONSTRAINT fk_project_service_service_type FOREIGN KEY (fk_service_type)
        REFERENCES clients.tb_service_type (pk_service_type)
        ON DELETE RESTRICT
);


-- =============================================================================
-- ALOCAÇÃO DIÁRIA DE COLABORADORES
-- Registra quais colaboradores trabalharam em qual projeto a cada dia.
-- Um colaborador pode estar em projetos diferentes em dias distintos,
-- mas não pode ser alocado duas vezes no mesmo projeto no mesmo dia
-- (garantido pela constraint uq_allocation_collaborator_day).
-- nr_hourly_rate_snapshot congela o valor/hora vigente no momento do registro,
-- preservando o histórico financeiro mesmo após reajustes futuros.
-- =============================================================================
CREATE TABLE clients.tb_project_daily_allocation
(
    pk_project_daily_allocation          UUID      NOT NULL,
    idx_project_daily_allocation         SERIAL    NOT NULL,
    fk_project                           UUID      NOT NULL,
    fk_collaborator                      UUID      NOT NULL,
    dt_work_date                         DATE      NOT NULL,
    nr_hours_worked                      NUMERIC(4, 2),  -- horas efetivas
    nr_hourly_rate_snapshot              NUMERIC(10, 2), -- valor/hora no momento
    tx_notes                             TEXT,
    bl_present                           BOOLEAN   NOT NULL DEFAULT TRUE,
    ts_allocated_collaborator_created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    ts_allocated_collaborator_updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT pk_project_daily_allocation PRIMARY KEY (pk_project_daily_allocation),
    CONSTRAINT uq_idx_project_daily_allocation UNIQUE (idx_project_daily_allocation),
    -- evita duplicidade de colaborador no mesmo projeto no mesmo dia
    CONSTRAINT uq_allocation_collaborator_day UNIQUE (fk_project, fk_collaborator, dt_work_date),
    CONSTRAINT fk_allocation_project FOREIGN KEY (fk_project)
        REFERENCES clients.tb_project (pk_project)
        ON DELETE CASCADE,
    CONSTRAINT fk_allocation_collaborator FOREIGN KEY (fk_collaborator)
        REFERENCES collaborators.tb_collaborator (pk_collaborator)
        ON DELETE CASCADE
);

-- Índices de performance para consultas de histórico por projeto e por colaborador
CREATE INDEX idx_tb_project_daily_allocation_fk_project
    ON clients.tb_project_daily_allocation (fk_project);
CREATE INDEX idx_tb_project_daily_allocation_fk_collaborator
    ON clients.tb_project_daily_allocation (fk_collaborator);