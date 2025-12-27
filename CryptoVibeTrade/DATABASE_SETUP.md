# Database Setup Guide

## PostgreSQL Installation

### macOS (Homebrew)
```bash
brew install postgresql@16
brew services start postgresql@16
```

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

### Windows
Download and install from: https://www.postgresql.org/download/windows/

## Database Creation

1. **Connect to PostgreSQL**
```bash
psql postgres
```

2. **Create Database and User**
```sql
CREATE DATABASE cryptovibetrade;
CREATE USER cvt_user WITH ENCRYPTED PASSWORD 'your_secure_password';
GRANT ALL PRIVILEGES ON DATABASE cryptovibetrade TO cvt_user;
\q
```

## Application Configuration

1. **Copy Environment Template**
```bash
cp .env.example .env
```

2. **Update .env with Database Credentials**
```
DATABASE_URL=postgresql://cvt_user:your_secure_password@localhost:5432/cryptovibetrade
```

3. **Initialize Database**
```bash
cargo run
```

The application will automatically:
- Create all necessary tables
- Set up indexes
- Configure triggers
- Initialize schema

## Database Schema

### users table
- `id` - UUID (primary key)
- `username` - VARCHAR(100) unique
- `email` - VARCHAR(255) unique
- `password_hash` - TEXT (argon2 hashed)
- `is_admin` - BOOLEAN
- `is_active` - BOOLEAN
- `created_at` - TIMESTAMP
- `updated_at` - TIMESTAMP

### login_sessions table
- `id` - UUID (primary key)
- `user_id` - UUID (foreign key)
- `token` - TEXT unique
- `expires_at` - TIMESTAMP
- `created_at` - TIMESTAMP
- `valid` - BOOLEAN

## Security Features

✅ **Password Hashing**: Argon2 (memory-hard algorithm)
✅ **Session Management**: 24-hour expiring tokens
✅ **SQL Injection Protection**: Parameterized queries
✅ **Credentials Never Exposed**: Environment variables only
✅ **Automatic Migration**: Schema updates on startup

## Testing Registration

1. Visit: http://127.0.0.1:3000/auth/register
2. Create account with username, email, password
3. Login at: http://127.0.0.1:3000/auth/login
4. Access dashboard: http://127.0.0.1:3000/admin/dashboard (admin only)

## Admin Access

Hardcoded admin credentials (fallback):
- Username: `fenrir`
- Password: `$4taN`

## Backup & Restore

### Backup
```bash
pg_dump -U cvt_user cryptovibetrade > backup.sql
```

### Restore
```bash
psql -U cvt_user cryptovibetrade < backup.sql
```

## Troubleshooting

**Connection refused?**
- Check PostgreSQL is running: `brew services list` or `systemctl status postgresql`
- Verify DATABASE_URL in .env

**Migration failed?**
- Drop and recreate database:
  ```sql
  DROP DATABASE cryptovibetrade;
  CREATE DATABASE cryptovibetrade;
  GRANT ALL PRIVILEGES ON DATABASE cryptovibetrade TO cvt_user;
  ```

**Password too weak?**
- Registration requires minimum 8 characters
- Database enforces Argon2 hashing
