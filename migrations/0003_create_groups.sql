-- up
CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

-- down
DROP TABLE groups;