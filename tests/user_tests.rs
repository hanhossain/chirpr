use chirpr::controllers;
use chirpr::database::Database;
use chirpr::models::{State, User, UserNoId};
use tide::http::{Request, Response, Url};
use tide::{Body, Result, StatusCode};

#[async_std::test]
async fn get_nonexistent_user() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    // get user
    let request = Request::get(Url::parse("http://localhost/api/users/nope")?);
    let response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::NotFound, response.status());

    Ok(())
}

#[async_std::test]
async fn create_user() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    let user = UserNoId {
        username: "testuser".to_string(),
    };

    // create user
    let mut request = Request::post(Url::parse("http://localhost/api/users")?);
    request.set_body(Body::from_json(&user)?);
    let mut response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::Created, response.status());
    let created_user = response.body_json::<User>().await?;
    assert_eq!(user.username, created_user.username);

    let request = Request::get(Url::parse(&format!(
        "http://localhost/api/users/{}",
        created_user.id
    ))?);
    let mut response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::Ok, response.status());

    let response_user = response.body_json::<User>().await?;
    assert_eq!(created_user, response_user);

    Ok(())
}

#[async_std::test]
async fn delete_user() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    let user = UserNoId {
        username: "testuser".to_string(),
    };

    // create user
    let mut request = Request::post(Url::parse("http://localhost/api/users")?);
    request.set_body(Body::from_json(&user)?);
    let mut response: Response = app.respond(request).await?;
    let user = response.body_json::<User>().await?;

    // delete user
    let url = Url::parse(&format!("http://localhost/api/users/{}", user.id))?;
    let request = Request::delete(url.clone());
    let response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::NoContent, response.status());

    // assert user doesn't exist
    let request = Request::get(url);
    let response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::NotFound, response.status());

    Ok(())
}

#[async_std::test]
async fn delete_nonexistent_user() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    // delete user
    let request = Request::delete(Url::parse("http://localhost/api/users/nope")?);
    let response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::NotFound, response.status());

    Ok(())
}

#[async_std::test]
async fn get_users() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    let usernames = vec!["testuser", "anotheruser"];

    let mut users = Vec::new();
    for username in usernames {
        let user = UserNoId {
            username: username.to_string(),
        };

        let mut request = Request::post(Url::parse("http://localhost/api/users")?);
        request.set_body(Body::from_json(&user)?);

        let mut response: Response = app.respond(request).await?;
        let user = response.body_json::<User>().await?;

        assert_eq!(username, user.username);
        users.push(user);
    }

    let request = Request::get(Url::parse("http://localhost/api/users")?);
    let mut response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::Ok, response.status());

    let actual_users = response.body_json::<Vec<User>>().await?;
    assert_eq!(users, actual_users);

    Ok(())
}

#[async_std::test]
async fn update_user() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    // create user
    let user = UserNoId {
        username: "myuser".to_string(),
    };
    let mut request = Request::post(Url::parse("http://localhost/api/users")?);
    request.set_body(Body::from_json(&user)?);
    let mut response: Response = app.respond(request).await?;
    let user: User = response.body_json().await?;

    // update user
    let updated_user = User {
        username: "newusername".to_string(),
        ..user
    };
    let url = Url::parse(&format!("http://localhost/api/users/{}", updated_user.id))?;
    let mut request = Request::put(url.clone());
    request.set_body(Body::from_json(&updated_user)?);
    let response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::Ok, response.status());

    // assert username changed
    let request = Request::get(url);
    let mut response: Response = app.respond(request).await?;
    let actual_user: User = response.body_json().await?;
    assert_eq!(updated_user, actual_user);

    Ok(())
}

#[async_std::test]
async fn update_nonexistent_user() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    // update user
    let user = User {
        id: "nouser".to_string(),
        username: "someuser".to_string(),
    };
    let url = Url::parse(&format!("http://localhost/api/users/{}", user.id))?;
    let mut request = Request::put(url.clone());
    request.set_body(Body::from_json(&user)?);
    let response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::NotFound, response.status());

    Ok(())
}

#[async_std::test]
async fn update_user_id_mismatch() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    // update user
    let user = User {
        id: "myid".to_string(),
        username: "someuser".to_string(),
    };
    let url = Url::parse("http://localhost/api/users/differentid")?;
    let mut request = Request::put(url.clone());
    request.set_body(Body::from_json(&user)?);
    let response: Response = app.respond(request).await?;
    assert_eq!(StatusCode::BadRequest, response.status());

    Ok(())
}
