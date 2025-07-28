-- Add up migration script here
CREATE TABLE record_type (record_type_id INTEGER PRIMARY KEY, name TEXT);

INSERT INTO
  record_type (record_type_id, name)
VALUES
  (1, "Income"),
  (2, "Outcome"),
  (3, "Transfer");

CREATE TABLE record (
  record_id INTEGER PRIMARY KEY,
  amount INTEGER NOT NULL,
  description TEXT,
  record_type INTEGER NOT NULL REFERENCES record_type (record_type_id),
  category_id INTEGER NULL REFERENCES category (category_id),
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
);

CREATE TABLE category (
  category_id INTEGER PRIMARY KEY,
  name TEXT UNIQUE NOT NULL,
  parent_id INTEGER NULL REFERENCES category (category_id),
  budget INTEGER
);
