CREATE TABLE IF NOT EXISTS tags (
  id TEXT PRIMARY KEY NOT NULL,
  label TEXT NOT NULL,
  value TEXT NOT NULL,
  display_name TEXT,
  status TEXT DEFAULT 'active',
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_tags_label_value ON tags(label, value);

CREATE TABLE IF NOT EXISTS repo_tag_map (
  repo_id TEXT NOT NULL,
  tag_id  TEXT NOT NULL,
  source  TEXT DEFAULT 'manual',
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT,
  PRIMARY KEY (repo_id, tag_id),
  FOREIGN KEY (repo_id) REFERENCES repos(id),
  FOREIGN KEY (tag_id) REFERENCES tags(id)
);

INSERT OR IGNORE INTO tags (id, label, value, display_name)
VALUES ('tag:UNTAG:untagged', 'UNTAG', 'untagged', '未分类');
