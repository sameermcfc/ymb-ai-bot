use axum::{routing::get, Json, Router, extract::Path};
use bson::doc;
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::{errors::Error, utils::generate_message::generate};


pub fn create_route() -> Router {
    Router::new().route("/message/:desc", get(get_message))
}

async fn get_message(Path(desc): Path<String>) -> Result<Json<Message>, Error> {
    debug!("Returning message");
    let response = generate(desc).await?;
    Ok(Json(Message { message: response }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String
}
