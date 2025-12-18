CREATE SCHEMA "collaborators";

CREATE TABLE "collaborators"."tb_collaborator" (
  "pk_collaborator" uuid PRIMARY KEY
);


ALTER TABLE "collaborators"."tb_collaborator" ADD FOREIGN KEY ("pk_collaborator") REFERENCES "users"."tb_user" ("pk_user");