-- Your SQL goes here
CREATE TABLE workflows_history (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  workflow_id VARCHAR(36) NOT NULL,
  content TEXT,
  is_success BOOLEAN NOT NULL DEFAULT 'f'
);