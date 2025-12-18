CREATE SCHEMA "clients";

CREATE TABLE "clients"."tb_client" (
  "pk_client" uuid PRIMARY KEY
);

ALTER TABLE "clients"."tb_client" ADD FOREIGN KEY ("pk_client") REFERENCES "users"."tb_user" ("pk_user");
