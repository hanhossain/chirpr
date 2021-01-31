use crate::database::Database;

mod database;
mod error;
mod models;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let db = Database::connect().await?;

    println!("----- Users -----");

    let users = db.get_users().await?;
    for user in users {
        println!("{} - {}", user.id, user.username);
    }

    println!("----- Creating User -----");
    db.create_user("john").await?;

    println!("----- Users -----");
    let users = db.get_users().await?;
    for user in users {
        println!("{} - {}", user.id, user.username);
    }

    Ok(())
}
