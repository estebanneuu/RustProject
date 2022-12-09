-- Your SQL goes here
CREATE TABLE commands (
  id SERIAL NOT NULL PRIMARY KEY,
  command VARCHAR(140) NOT NULL,
  agent_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL,
  ip VARCHAR NOT NULL,
  done boolean NOT NULL DEFAULT false
)