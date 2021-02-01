use chirpr::controllers;
use chirpr::database::Database;
use chirpr::models::State;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let database = Database::connect("sqlite://chirpr.db").await?;
    let app = controllers::build_routes(State { database });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
