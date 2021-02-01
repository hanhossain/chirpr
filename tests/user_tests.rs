use chirpr::controllers;
use chirpr::database::Database;
use chirpr::models::{State, User, UserNoId};
use tide::http::{Request, Response, Url};
use tide::{Body, Result, StatusCode};

#[async_std::test]
async fn create_user() -> Result<()> {
    let database = Database::in_memory().await?;
    let app = controllers::build_routes(State { database });

    let user = UserNoId {
        username: "testuser".to_string(),
    };

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
