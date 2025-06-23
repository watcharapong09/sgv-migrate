# sgv-migrate

A simple PostgreSQL migration tool written in Rust.

Supports applying, rolling back, and listing SQL migrations stored in the `migrations/` directory. Uses a migrations table to track which migrations have already been applied.

---

## 🧰 Features

- ✅ Environment-based `.env` loading (`--env development|test|production`)
- 📈 Tracks applied migrations in the database
- ⬆️ Apply all pending migrations
- ⬇️ Roll back one or more migrations
- 📃 List applied and pending migrations

---

## 📦 Usage

```bash
cargo run -- [OPTIONS] <COMMAND>