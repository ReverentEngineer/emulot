BEGIN;
CREATE TABLE IF NOT EXISTS system_config (
	name TEXT UNIQUE NOT NULL,
	config TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS system_config_name ON system_config(name);
COMMIT;
