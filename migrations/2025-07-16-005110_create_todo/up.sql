-- Your SQL goes here

CREATE TABLE todos
(
    id          VARCHAR(255) NOT NULL,
    title       VARCHAR(255) NOT NULL,
    description LONGTEXT,
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP,
    CONSTRAINT PK_Todos PRIMARY KEY (id)
)
