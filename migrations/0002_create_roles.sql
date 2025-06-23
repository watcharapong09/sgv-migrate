-- up
CREATE TABLE roles (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

-- down
DROP TABLE roles;