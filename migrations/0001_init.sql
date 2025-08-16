PRAGMA foreign_keys = ON;

CREATE TABLE projects (
  id            TEXT PRIMARY KEY,                 -- uuid
  name          TEXT NOT NULL UNIQUE,
  description   TEXT,
  archived      INTEGER NOT NULL DEFAULT 0,
  created_at    TEXT NOT NULL                     -- RFC3339
);

CREATE TABLE tasks (
  id            TEXT PRIMARY KEY,                 -- uuid
  project_id    TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  name          TEXT NOT NULL,
  description   TEXT,
  archived      INTEGER NOT NULL DEFAULT 0,
  created_at    TEXT NOT NULL
);

CREATE UNIQUE INDEX ux_task_project_name ON tasks(project_id, name);

CREATE TABLE time_entries (
  id                TEXT PRIMARY KEY,            -- uuid
  task_id           TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  start_time        TEXT NOT NULL,               -- RFC3339 (UTC)
  end_time          TEXT,                        -- RFC3339 (UTC), NULL while running
  duration_seconds  INTEGER,                     -- computed on stop
  local_date        TEXT NOT NULL,               -- YYYY-MM-DD (user tz at start)
  created_at        TEXT NOT NULL
);

-- Only one running timer at a time across ALL tasks:
CREATE TABLE active_timer (
  singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
  time_entry_id TEXT UNIQUE REFERENCES time_entries(id) ON DELETE SET NULL
);

INSERT INTO active_timer(singleton, time_entry_id) VALUES (1, NULL);
