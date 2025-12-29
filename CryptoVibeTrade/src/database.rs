use anyhow::Result;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Database { pool })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(100) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                full_name VARCHAR(255),
                birth_date DATE,
                password_hash TEXT NOT NULL,
                is_admin BOOLEAN DEFAULT FALSE NOT NULL,
                is_active BOOLEAN DEFAULT TRUE NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
            );

            ALTER TABLE users
                ADD COLUMN IF NOT EXISTS full_name VARCHAR(255),
                ADD COLUMN IF NOT EXISTS birth_date DATE;

            CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
            CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

            CREATE TABLE IF NOT EXISTS login_sessions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                token TEXT UNIQUE NOT NULL,
                expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
                valid BOOLEAN DEFAULT TRUE NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_login_sessions_token ON login_sessions(token);
            CREATE INDEX IF NOT EXISTS idx_login_sessions_user_id ON login_sessions(user_id);

            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql';

            DROP TRIGGER IF EXISTS update_users_updated_at ON users;
            CREATE TRIGGER update_users_updated_at
                BEFORE UPDATE ON users
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
            "#,
        )
        .execute(&self.pool)
        .await?;

        tracing::info!("Database migrations completed successfully");
        Ok(())
    }

    pub async fn seed_admin_user(&self) -> Result<()> {
        // Check if admin user already exists
        if self.get_user_by_username("fenrir").await?.is_some() {
            tracing::info!("Admin user 'fenrir' already exists, skipping seed");
            return Ok(());
        }

        // Create admin user
        self.create_user("fenrir", "fenrir@cvt.local", None, None, "$4t4N", true)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create admin user: {}", e))?;

        tracing::info!("Admin user 'fenrir' created successfully");
        Ok(())
    }

    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Create a new user with optional profile details.
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        full_name: Option<&str>,
        birth_date: Option<NaiveDate>,
        password: &str,
        is_admin: bool,
    ) -> Result<User> {
        let password_hash = Self::hash_password(password)?;

        let row = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, full_name, birth_date, password_hash, is_admin)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(full_name)
        .bind(birth_date)
        .bind(&password_hash)
        .bind(is_admin)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1 AND is_active = TRUE",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND is_active = TRUE")
                .bind(email)
                .fetch_optional(&self.pool)
                .await?;
        Ok(user)
    }

    pub async fn verify_credentials(&self, username: &str, password: &str) -> Result<Option<User>> {
        if let Some(user) = self.get_user_by_username(username).await? {
            if Self::verify_password(password, &user.password_hash)? {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }

    pub async fn create_session(&self, user_id: &Uuid, expires_in_hours: i64) -> Result<String> {
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(expires_in_hours);

        sqlx::query("INSERT INTO login_sessions (user_id, token, expires_at) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind(&token)
            .bind(expires_at)
            .execute(&self.pool)
            .await?;

        Ok(token)
    }

    pub async fn validate_session(&self, token: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT u.*
            FROM users u
            INNER JOIN login_sessions ls ON u.id = ls.user_id
            WHERE ls.token = $1
                AND ls.valid = TRUE
                AND ls.expires_at > NOW()
                AND u.is_active = TRUE
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                email: row.try_get("email")?,
                full_name: row.try_get("full_name")?,
                birth_date: row.try_get("birth_date")?,
                password_hash: row.try_get("password_hash")?,
                is_admin: row.try_get("is_admin")?,
                is_active: row.try_get("is_active")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn invalidate_session(&self, token: &str) -> Result<()> {
        sqlx::query("UPDATE login_sessions SET valid = FALSE WHERE token = $1")
            .bind(token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for User {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            full_name: row.try_get("full_name")?,
            birth_date: row.try_get("birth_date")?,
            password_hash: row.try_get("password_hash")?,
            is_admin: row.try_get("is_admin")?,
            is_active: row.try_get("is_active")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
