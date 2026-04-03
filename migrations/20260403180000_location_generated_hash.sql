-- Replace nr_hash with a GENERATED ALWAYS AS column using md5().
-- The hash is computed by the database from the address fields,
-- eliminating application-side hash logic and cross-version instability.
--
-- hashtext() is STABLE (not IMMUTABLE), so we use md5() which IS
-- immutable and therefore valid for generated columns.

-- Drop the existing nr_hash column (this cascades the unique constraint)
ALTER TABLE locations.tb_location DROP COLUMN IF EXISTS nr_hash;

-- Add the generated column using md5 (immutable).
-- Takes the first 16 hex chars of md5 (64 bits) and casts to bigint.
ALTER TABLE locations.tb_location
    ADD COLUMN nr_hash BIGINT GENERATED ALWAYS AS (
        ('x' || substr(
            md5(concat_ws('|', tx_street, tx_number, tx_city, tx_state, tx_zipcode)),
            1, 16
        ))::bit(64)::bigint
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
