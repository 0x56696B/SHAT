-- DROP TABLE IF EXISTS shat_database_api;
CREATE TABLE IF NOT EXISTS People(
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE
);

CREATE TABLE IF NOT EXISTS Messages(
    id SERIAL PRIMARY KEY,
    message TEXT NOT NULL,
    issuer_id INT NOT NULL,
    FOREIGN KEY(issuer_id) REFERENCES People(id)
);

CREATE TABLE IF NOT EXISTS Chats(
    id SERIAL PRIMARY KEY,
    issuer_id INT NOT NULL,
    message_id INT NOT NULL,
    FOREIGN KEY(issuer_id) REFERENCES People(id),
    FOREIGN KEY(message_id) REFERENCES Messages(id)
);
