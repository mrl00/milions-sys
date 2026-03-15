-- =============================================================================
-- SCHEMA LOCATIONS
-- Centraliza endereços reutilizáveis por qualquer entidade do sistema
-- (clientes, projetos, colaboradores etc.).
-- Os dados de tx_ibge, tx_gia, tx_ddd e tx_siafi são códigos oficiais
-- retornados pela API ViaCEP e armazenados para rastreabilidade fiscal e
-- integrações com sistemas de governo.
-- =============================================================================
CREATE SCHEMA IF NOT EXISTS locations;


-- =============================================================================
-- LOCALIZAÇÃO
-- Endereço físico completo, enriquecido com metadados do ViaCEP.
-- Projetado para ser referenciado por múltiplas entidades via FK,
-- evitando duplicação de dados de endereço em cada tabela.
-- tx_unit representa o tipo de logradouro (Rua, Avenida, Travessa etc.).
-- tx_ibge → código IBGE do município.
-- tx_gia  → código GIA (uso fiscal, SP). Nullable pois só existe em SP.
-- tx_ddd  → código de discagem direta a distância da cidade.
-- tx_siafi → código SIAFI do município (sistema de finanças do governo federal).
-- =============================================================================
CREATE TABLE locations.tb_location
(
    pk_location            UUID                    NOT NULL,
    idx_location           SERIAL                  NOT NULL,
    tx_unit                VARCHAR(64)             NOT NULL, -- tipo de logradouro: Rua, Av., Travessa etc.
    tx_street              VARCHAR(128)            NOT NULL,
    tx_number              VARCHAR(128)            NOT NULL,
    tx_address_complement  VARCHAR(256),
    tx_neighborhood        VARCHAR(128)            NOT NULL,
    tx_public_space        VARCHAR(128)            NOT NULL,
    tx_locality            VARCHAR(128)            NOT NULL,
    tx_city                VARCHAR(128)            NOT NULL,
    tx_state               VARCHAR(2)              NOT NULL,
    tx_region              VARCHAR(64)             NOT NULL,
    tx_zipcode             VARCHAR(9)              NOT NULL,
    tx_ddd                 VARCHAR(3)              NOT NULL,
    tx_ibge                VARCHAR(16)             NOT NULL,
    tx_gia                 VARCHAR(16),                      -- exclusivo para municípios de SP
    tx_siafi               VARCHAR(8)              NOT NULL,
    ts_location_created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    ts_location_updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    CONSTRAINT pk_location PRIMARY KEY (pk_location),
    CONSTRAINT uq_idx_location UNIQUE (idx_location)
);