CREATE TABLE IF NOT EXISTS tags (
  id text PRIMARY KEY,
  label text NOT NULL,
  value text NOT NULL,
  display_name text,
  status text DEFAULT 'active',
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_tags_label_value ON tags(label, value);

CREATE TABLE IF NOT EXISTS repo_tag_map (
  repo_id text NOT NULL,
  tag_id  text NOT NULL,
  source  text DEFAULT 'manual',
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  PRIMARY KEY (repo_id, tag_id),
  FOREIGN KEY (repo_id) REFERENCES repos(id),
  FOREIGN KEY (tag_id) REFERENCES tags(id)
);

INSERT INTO tags (id, label, value, display_name)
VALUES ('tag:UNTAG:untagged', 'UNTAG', 'untagged', '未分类')
ON CONFLICT DO NOTHING;
