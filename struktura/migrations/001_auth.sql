-- Migration: Secure Authentication and Usage Analysis Setup

-- Phase 1: Connect and deploy extensions
-- This is often the first step in a migration.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS experience_level (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    level_name VARCHAR(50) NOT NULL UNIQUE
);

INSERT INTO experience_level (level_name) VALUES
('beginner'),
('intermediate'),
('advanced');

-- Phase 2: Primary fortifications (Secure Users Table)
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL UNIQUE,
    -- CRITICAL SECURITY UPGRADE: Switched from fixed-length hash (like SHA-512) 
    -- to VARCHAR to store memory-hard Argon2id hashes.
    -- Argon2 hashes include the algorithm, version, salt, and parameters, 
    -- making them much longer and resistant to brute-force attacks.
    hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_pro BOOLEAN DEFAULT FALSE,
    fav_experience_level VARCHAR(50) REFERENCES experience_level(level_name), 
    
    CONSTRAINT username_not_empty CHECK (username != ''),
    CONSTRAINT username_format CHECK (username ~ '^[a-zA-Z0-9_-]+$')
);

-- Phase 3: Intelligence gathering infrastructure (Usage Metrics)
CREATE TABLE IF NOT EXISTS usage_metrics (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    accessed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    feature_name VARCHAR(100) NOT NULL,
    experience_level_used VARCHAR(50) NOT NULL REFERENCES experience_level(level_name), -- Useful for segmenting usage analysis
    
    CONSTRAINT feature_not_empty CHECK (feature_name != '')
);

-- Phase 4: Strategic indices for rapid reconnaissance
CREATE INDEX idx_usage_metrics_user ON usage_metrics(user_id);
CREATE INDEX idx_usage_metrics_feature ON usage_metrics(feature_name);
CREATE INDEX idx_usage_metrics_accessed ON usage_metrics(accessed_at DESC);
-- Composite index for common queries: filtering by feature and ordering by time.
CREATE INDEX idx_usage_metrics_composite ON usage_metrics(feature_name, accessed_at DESC);
-- Composite index for user-specific statistics queries
CREATE INDEX idx_usage_metrics_user_feature ON usage_metrics(user_id, feature_name);

-- Phase 5: Useful metrics view (because that's the whole point of analysis)
CREATE OR REPLACE VIEW feature_usage_stats AS
SELECT 
    feature_name,
    COUNT(*) as access_count,
    COUNT(DISTINCT user_id) as unique_users,
    MAX(accessed_at) as last_accessed,
    -- Calculate the most common experience level using mode() within a group
    (SELECT mode() WITHIN GROUP (ORDER BY experience_level_used) FROM usage_metrics um2 WHERE um2.feature_name = um.feature_name) as common_experience_level
FROM 
    usage_metrics um
GROUP BY 
    feature_name
ORDER BY
    access_count DESC;