CREATE SCHEMA IF NOT EXISTS "jobs";

CREATE TABLE "jobs"."tb_collaborator" (
  "pk_collaborator" uuid PRIMARY KEY,
  "fk_projects" uuid
);

CREATE TABLE "jobs"."tb_tasks" (
  "pk_collaborator" uuid PRIMARY KEY,
  "tx_name" varchar(128) NOT NULL,
  "tx_status" varchar(32) NOT NULL,
  "fk_projects" uuid
);

CREATE TABLE "jobs"."tb_job" (
  "pk_projects" uuid PRIMARY KEY,
  "tx_name" varchar(64) NOT NULL,
  "tx_status" varchar NOT NULL,
  "fk_address" uuid,
  "fk_client" uuid
);

ALTER TABLE "jobs"."tb_collaborator" ADD FOREIGN KEY ("fk_projects") REFERENCES "jobs"."tb_job" ("pk_projects");

ALTER TABLE "jobs"."tb_tasks" ADD FOREIGN KEY ("fk_projects") REFERENCES "jobs"."tb_job" ("pk_projects");

ALTER TABLE "jobs"."tb_job" ADD FOREIGN KEY ("fk_address") REFERENCES "locations"."tb_location" ("pk_location");

ALTER TABLE "jobs"."tb_job" ADD FOREIGN KEY ("fk_client") REFERENCES "clients"."tb_client" ("pk_client");

ALTER TABLE "jobs"."tb_collaborator" ADD FOREIGN KEY ("pk_collaborator") REFERENCES "collaborators"."tb_collaborator" ("pk_collaborator");
