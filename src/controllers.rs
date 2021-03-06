use crate::models::State;
use tide::Server;

mod user_controller;

pub fn build_routes(state: State) -> Server<State> {
    let mut app = tide::with_state(state);

    app.at("/api/users")
        .get(user_controller::get_users)
        .post(user_controller::create_user);

    app.at("/api/users/:user_id")
        .delete(user_controller::delete_user)
        .get(user_controller::get_user)
        .put(user_controller::update_user);

    app
}
