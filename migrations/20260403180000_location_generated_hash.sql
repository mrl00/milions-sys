-- Replace nr_hash with a GENERATED ALWAYS AS column using hashtext.
-- The hash is computed by the database from the address fields,
-- eliminating application-side hash logic and cross-version instability.

-- Drop the existing nr_hash column (this cascades the unique constraint)
ALTER TABLE locations.tb_location DROP COLUMN IF EXISTS nr_hash;

-- Add the generated column
ALTER TABLE locations.tb_location
    ADD COLUMN nr_hash BIGINT GENERATED ALWAYS AS (
        hashtext(concat_ws('|', tx_street, tx_number, tx_city, tx_state, tx_zipcode))
    ) STORED;

-- Add the unique constraint (idempotent)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'uq_nr_hash'
    ) THEN
        ALTER TABLE locations.tb_location ADD CONSTRAINT uq_nr_hash UNIQUE (nr_hash);
    END IF;
END $$;
