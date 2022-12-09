-- Your SQL goes here
CREATE TABLE results (
  id SERIAL NOT NULL PRIMARY KEY,
  command_id INTEGER NOT NULL,
  agent_id INTEGER NOT NULL,
  result_content VARCHAR NOT NULL,
  done_at TIMESTAMP NOT NULL
)