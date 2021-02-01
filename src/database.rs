use crate::error::Error;
use crate::models::User;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect() -> Result<Database, Error> {
        let pool = SqlitePool::connect("chirpr.db").await?;
        Ok(Database { pool })
    }

    pub async fn create_user(&self, username: &str) -> Result<User, Error> {
        let id = Uuid::new_v4().to_string();

        sqlx::query("insert into users (id, username) values (?, ?)")
            .bind(&id)
            .bind(username)
            .execute(&self.pool)
            .await?;

        Ok(User {
            id,
            username: username.to_string(),
        })
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("select id, username from users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }
}
