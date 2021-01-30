use crate::database::Database;

mod database;
mod error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let db = Database::connect().await?;

    let user = db.create_user("han").await?;
    println!("{:?}", user);

    let users = db.get_users().await?;
    println!("{:?}", users);

    Ok(())
}
