# sgv-migrate

A simple CLI tool for managing PostgreSQL migrations. Inspired by tools like `sqlx migrate`, but designed for projects that want a straightforward and dependency-light solution.

## Features

- ✅ Apply migrations (`up`)
- 🔽 Roll back migrations (`down`)
- 📋 List pending migrations (`list`)
- 🧩 Environment-based configuration (`.env`, `.env.production`, `.env.test`)
- 🏷️ Schema-aware (via `MIGRATION_SCHEMA` env)

## Installation

```bash
cargo install sgv-migrate
````

## Configuration

Create a `.env` file (or `.env.production`, `.env.test`) in your project with the following:

```env
MIGRATION_DATABASE_URL=postgres://user:password@localhost:5432/yourdb
MIGRATION_SCHEMA=public  # Optional, defaults to "public"
```

## Migrations

Put your migration files in a folder called `migrations/`.

Each `.sql` file should contain two sections separated by comments:

```sql
-- up
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- down
DROP TABLE users;
```

Name your files in lexicographical order so they apply in the right sequence:

```
migrations/
├── 0001_create_users.sql
├── 0002_add_roles.sql
```

✅ Migrations with names **lexically less than the latest applied** (e.g., `0000_...`) will be skipped.

## Commands

```bash
sgv-migrate up             # Apply all pending migrations
sgv-migrate down           # Revert the last migration
sgv-migrate down --step=2  # Revert the last 2 migrations
sgv-migrate down --step=-1 # Revert all applied migrations
sgv-migrate list           # Show pending migrations
```

## License

MIT © 2025 [Watcharapong Essaranuwatanakul](https://github.com/watcharapong09)
