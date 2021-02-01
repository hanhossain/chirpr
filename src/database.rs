use crate::error::Error;
use crate::models::User;
use sqlx::SqlitePool;
use uuid::Uuid;

const SCHEMA: &str = include_str!("../schema/schema.sql");

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(uri: &str) -> Result<Database, Error> {
        let pool = SqlitePool::connect(uri).await?;

        sqlx::query(SCHEMA).execute(&pool).await?;

        Ok(Database { pool })
    }

    pub async fn in_memory() -> Result<Database, Error> {
        Database::connect("sqlite::memory:").await
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

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>("select id, username from users where id == ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>("select id, username from users where username == ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn update_user(&self, user: User) -> Result<Option<User>, Error> {
        let mut transaction = self.pool.begin().await?;

        if let None = sqlx::query_as::<_, User>("select id, username from users where id == ?")
            .bind(&user.id)
            .fetch_optional(&mut transaction)
            .await?
        {
            return Ok(None);
        }

        sqlx::query("update users set username = ? where id == ?")
            .bind(&user.username)
            .bind(&user.id)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(Some(user))
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<bool, Error> {
        let mut transaction = self.pool.begin().await?;

        if let None = sqlx::query_as::<_, User>("select id, username from users where id == ?")
            .bind(user_id)
            .fetch_optional(&mut transaction)
            .await?
        {
            return Ok(false);
        }

        sqlx::query("delete from users where id == ?")
            .bind(user_id)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(true)
    }
}
