use crate::database::Database;
use crate::models::State;

mod controllers;
mod database;
mod error;
mod models;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let database = Database::connect("sqlite://chirpr.db").await?;
    let app = controllers::build_routes(State { database });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
