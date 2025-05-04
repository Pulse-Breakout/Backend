-- Drop all tables in the proper order to handle foreign key constraints
DROP TABLE IF EXISTS depositor;
DROP TABLE IF EXISTS content;
DROP TABLE IF EXISTS communities;
DROP TABLE IF EXISTS users;

-- Drop additional database objects if they exist
DROP TYPE IF EXISTS _sqlx_migrations;
DROP TABLE IF EXISTS _sqlx_migrations;