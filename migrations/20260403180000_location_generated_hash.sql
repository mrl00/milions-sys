-- Replace nr_hash with a GENERATED ALWAYS AS column.
-- The hash is computed by the database from the address fields,
-- eliminating application-side hash logic and cross-version instability.
--
-- PostgreSQL requires GENERATED ALWAYS AS expressions to be IMMUTABLE.
-- concat_ws() with text inputs can inherit collation, making it non-immutable.
-- We wrap the logic in an explicit IMMUTABLE function to satisfy the check.

-- Drop the existing nr_hash column (this cascades the unique constraint)
ALTER TABLE locations.tb_location DROP COLUMN IF EXISTS nr_hash;

-- Create an immutable function to compute the hash
CREATE OR REPLACE FUNCTION locations.compute_location_hash(
    p_street text,
    p_number text,
    p_city text,
    p_state text,
    p_zipcode text
) RETURNS bigint
IMMUTABLE
LANGUAGE sql
AS $$
    SELECT ('x' || substr(md5(p_street || '|' || p_number || '|' || p_city || '|' || p_state || '|' || p_zipcode), 1, 16))::bit(64)::bigint
$$;

-- Add the generated column
ALTER TABLE locations.tb_location
    ADD COLUMN nr_hash BIGINT GENERATED ALWAYS AS (
        locations.compute_location_hash(tx_street, tx_number, tx_city, tx_state, tx_zipcode)
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
