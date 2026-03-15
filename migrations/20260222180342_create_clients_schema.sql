CREATE SCHEMA IF NOT EXISTS "clients";
CREATE TABLE "clients"."tb_client"
(
    "pk_client"            UUID PRIMARY KEY,
    "idx_client"           SERIAL UNIQUE           NOT NULL,
    "tx_name"              VARCHAR(64)             NOT NULL,
    "tx_status"            VARCHAR(32)             NOT NULL,
    "ts_client_created_at" TIMESTAMP DEFAULT NOW() NOT NULL,
    "ts_client_updated_at" TIMESTAMP DEFAULT NOW() NOT NULL
);
CREATE TABLE "clients"."tb_client_contact"
(
    "pk_client_contact"            UUID PRIMARY KEY,
    "idx_client_contact"           SERIAL UNIQUE           NOT NULL,
    "fk_client"                    UUID UNIQUE             NOT NULL,
    "fk_contact"                   UUID UNIQUE             NOT NULL,
    "ts_client_contact_created_at" TIMESTAMP DEFAULT NOW() NOT NULL,
    "ts_client_contact_updated_at" TIMESTAMP DEFAULT NOW() NOT NULL
);
ALTER TABLE "clients"."tb_client_contact"
    ADD FOREIGN KEY ("fk_client") REFERENCES "clients"."tb_client" ("pk_client")
        ON DELETE CASCADE;
ALTER TABLE "clients"."tb_client_contact"
    ADD FOREIGN KEY ("fk_contact") REFERENCES "contacts"."tb_contact" ("pk_contact")
        ON DELETE RESTRICT;
CREATE INDEX idx_tb_client_contact_fk_client
    ON "clients"."tb_client_contact" ("fk_client");
CREATE INDEX idx_tb_client_contact_fk_contact
    ON "clients"."tb_client_contact" ("fk_contact");
CREATE TABLE "clients"."tb_client_address"
(
    "pk_client_address"            UUID PRIMARY KEY,
    "idx_client_address"           SERIAL UNIQUE           NOT NULL,
    "fk_client"                    UUID                    NOT NULL,
    "fk_address"                   UUID                    NOT NULL,
    "ts_client_address_created_at" TIMESTAMP DEFAULT NOW() NOT NULL,
    "ts_client_address_updated_at" TIMESTAMP DEFAULT NOW() NOT NULL
);
ALTER TABLE "clients"."tb_client_address"
    ADD FOREIGN KEY ("fk_client") REFERENCES "clients"."tb_client" ("pk_client")
        ON DELETE CASCADE;
ALTER TABLE "clients"."tb_client_address"
    ADD FOREIGN KEY ("fk_address") REFERENCES "locations"."tb_location" ("pk_location")
        ON DELETE RESTRICT;
CREATE INDEX idx_tb_client_address_fk_client
    ON "clients"."tb_client_address" ("fk_client");
CREATE INDEX idx_tb_client_address_fk_address
    ON "clients"."tb_client_address" ("fk_address");
CREATE TABLE "clients"."tb_project"
(
    "pk_project"            UUID PRIMARY KEY,
    "idx_project"           SERIAL UNIQUE           NOT NULL,
    "tx_name"               VARCHAR(64)             NOT NULL,
    "tx_status"             VARCHAR(32)             NOT NULL,
    "fk_address"            UUID                    NOT NULL,
    "fk_client"             UUID                    NOT NULL,
    "ts_project_created_at" TIMESTAMP DEFAULT NOW() NOT NULL,
    "ts_project_updated_at" TIMESTAMP DEFAULT NOW() NOT NULL
);
ALTER TABLE "clients"."tb_project"
    ADD FOREIGN KEY ("fk_address") REFERENCES "locations"."tb_location" ("pk_location")
        ON DELETE RESTRICT;
ALTER TABLE "clients"."tb_project"
    ADD FOREIGN KEY ("fk_client") REFERENCES "clients"."tb_client" ("pk_client")
        ON DELETE CASCADE;
CREATE INDEX idx_tb_project_fk_client
    ON "clients"."tb_project" ("fk_client");
CREATE INDEX idx_tb_project_fk_address
    ON "clients"."tb_project" ("fk_address");
CREATE TABLE "clients"."tb_allocated_collaborator"
(
    "pk_allocated_collaborator"            UUID PRIMARY KEY,
    "idx_allocated_collaborator"           SERIAL UNIQUE           NOT NULL,
    "fk_collaborator"                      UUID                    NOT NULL,
    "fk_project"                           UUID                    NOT NULL,
    "ts_allocated_collaborator_created_at" TIMESTAMP DEFAULT NOW() NOT NULL,
    "ts_allocated_collaborator_updated_at" TIMESTAMP DEFAULT NOW() NOT NULL
);
ALTER TABLE "clients"."tb_allocated_collaborator"
    ADD FOREIGN KEY ("fk_project") REFERENCES "clients"."tb_project" ("pk_project")
        ON DELETE CASCADE;
ALTER TABLE "clients"."tb_allocated_collaborator"
    ADD FOREIGN KEY ("fk_collaborator") REFERENCES "collaborators"."tb_collaborator" ("pk_collaborator")
        ON DELETE CASCADE;
CREATE INDEX idx_tb_allocated_collaborator_fk_project
    ON "clients"."tb_allocated_collaborator" ("fk_project");
CREATE INDEX idx_tb_allocated_collaborator_fk_collaborator
    ON "clients"."tb_allocated_collaborator" ("fk_collaborator");
