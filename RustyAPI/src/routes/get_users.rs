use axum::{Json, extract::State};

use crate::models::user::User;
use std::sync::Arc;
use tokio::sync::Mutex;

type Storage = Mutex<Vec<User>>;
type MainState = State<Arc<Storage>>;


pub async fn get_users(State(storage): MainState) -> Json<Vec<User>> {
    Json(storage.lock().await.clone())
}