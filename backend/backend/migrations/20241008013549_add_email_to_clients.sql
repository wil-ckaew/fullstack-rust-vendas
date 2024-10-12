-- Add migration script here
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns 
        WHERE table_name='clients' AND column_name='email'
    ) THEN
        ALTER TABLE clients ADD COLUMN email VARCHAR(100) NOT NULL UNIQUE;
    END IF;
END $$;
