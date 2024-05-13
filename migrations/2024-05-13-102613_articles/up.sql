-- Your SQL goes here
CREATE TABLE articles (
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_by INTEGER,
    created_on TIMESTAMP NOT NULL
)