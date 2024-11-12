mod hello_world;
mod get_users;

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    routing::{get, post},
    Router
};

use hello_world::hello_world;
use get_users::get_users;
use crate::models::user::User; // TODO: remove 'crate' and understand the weird import here

pub fn create_routes() -> Router {
    let init_user = vec![
        User{
            id: "0".to_string(),
            username: "None".to_string(),
            password: "None".to_string(),
            premium: false
        }
    ];

    let storage = Arc::new(Mutex::new(init_user));

    Router::new()
    .route("/", get(hello_world))
    .route("/users", get(get_users))
    .with_state(storage)
}