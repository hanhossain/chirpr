use crate::models::{State, User, UserNoId};
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

pub async fn update_user(mut req: Request<State>) -> Result {
    let user = req.body_json::<User>().await?;
    let user_id = req.param("user_id")?;

    if user_id != user.id {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let response = match req.state().database.update_user(user).await? {
        Some(user) => Response::builder(StatusCode::Ok)
            .body(Body::from_json(&user)?)
            .build(),
        None => Response::new(StatusCode::NotFound),
    };

    Ok(response)
}
