-- Add migration script here
CREATE TABLE IF NOT EXISTS app_users (
id BIGSERIAL PRIMARY KEY,
username TEXT NOT NULL,
email TEXT NOT NULL,
password TEXT NOT NULL
)