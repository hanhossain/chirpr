use crate::database::Database;
use crate::models::UserNoId;
use tide::{Body, Request, Response, StatusCode};

mod database;
mod error;
mod models;

#[derive(Clone)]
struct State {
    database: Database,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let database = Database::connect().await?;
    let mut app = tide::with_state(State { database });
    app.at("/api/users").get(get_users).post(create_user);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_users(req: Request<State>) -> tide::Result {
    let users = req.state().database.get_users().await?;
    let response = Response::builder(StatusCode::Ok)
        .body(Body::from_json(&users)?)
        .build();

    Ok(response)
}

async fn create_user(mut req: Request<State>) -> tide::Result {
    let user = req.body_json::<UserNoId>().await?;

    let user = req.state().database.create_user(&user.username).await?;
    let response = Response::builder(StatusCode::Created)
        .body(Body::from_json(&user)?)
        .build();

    Ok(response)
}
