use crate::models::{State, UserNoId};
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_users(req: Request<State>) -> Result {
    let users = req.state().database.get_users().await?;

    let response = Response::builder(StatusCode::Ok)
        .body(Body::from_json(&users)?)
        .build();

    Ok(response)
}

pub async fn create_user(mut req: Request<State>) -> Result {
    let user = req.body_json::<UserNoId>().await?;
    let database = &req.state().database;

    if let Some(user) = database.get_user(&user.username).await? {
        let response = Response::builder(StatusCode::Ok)
            .body(Body::from_json(&user)?)
            .build();

        return Ok(response);
    }

    let user = database.create_user(&user.username).await?;

    let response = Response::builder(StatusCode::Created)
        .body(Body::from_json(&user)?)
        .build();

    Ok(response)
}
