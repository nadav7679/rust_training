use axum::{extract::{self, State}, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::models::user::User;
use std::sync::Arc;
use tokio::sync::Mutex;


type Storage = Mutex<Vec<User>>;
type MainState = State<Arc<Storage>>;

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

pub async fn new_user(State(storage): MainState, extract::Json(body): extract::Json<NewUser>) -> Json<User> {
    let mut users = storage.lock().await;

    let new_user = User{
        id: Uuid::new_v4().to_string(),
        username: body.username,
        password: body.password,
        premium: false
    };
    users.push(new_user.clone());

    Json(new_user)
} 