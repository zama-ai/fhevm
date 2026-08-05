-- The GPU key-generation path stores compressed keysets in the connector
-- response tables. Keep the database enum in sync with KeyType::CompressedKeySet.
ALTER TYPE key_type ADD VALUE IF NOT EXISTS 'CompressedKeySet';
