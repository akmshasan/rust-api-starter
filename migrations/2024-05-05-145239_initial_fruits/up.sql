-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "fruits" (
  "id" bigserial PRIMARY KEY,
  "name" varchar NOT NULL,
  "color" varchar NOT NULL,
  "price" float NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE INDEX ON "fruits" ("name");

