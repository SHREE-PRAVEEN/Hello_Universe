-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";    -- trigram similarity for fuzzy search
CREATE EXTENSION IF NOT EXISTS "unaccent";   -- normalize accented characters in search

