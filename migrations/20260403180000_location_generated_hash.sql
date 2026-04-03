-- Replace nr_hash with a GENERATED ALWAYS AS column using hashtext.
-- The hash is computed by the database from the address fields,
-- eliminating application-side hash logic and cross-version instability.

ALTER TABLE locations.tb_location DROP COLUMN nr_hash;
ALTER TABLE locations.tb_location DROP CONSTRAINT uq_nr_hash;

ALTER TABLE locations.tb_location
    ADD COLUMN nr_hash BIGINT GENERATED ALWAYS AS (
        hashtext(concat_ws('|', tx_street, tx_number, tx_city, tx_state, tx_zipcode))
    ) STORED;

ALTER TABLE locations.tb_location ADD CONSTRAINT uq_nr_hash UNIQUE (nr_hash);
