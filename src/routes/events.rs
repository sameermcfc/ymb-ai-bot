use axum::routing::get;
use axum::{Router, extract::Query, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::utils::calendar::StructuredEvent;
use serde_json::Value;
use tracing::{info, error};
use crate::{errors::Error, utils::calendar::generate_structured_calendar_data};

pub fn create_route() -> Router {
    Router::new().route("/events", get(events_handler))
}


async fn events_handler(Query(params): Query<MonthQuery>) ->  Result<impl IntoResponse, impl IntoResponse> {
     info!("Fetching structured events for month: {}", params.month);

    let events = match generate_structured_calendar_data(params.month).await {
        Ok(events) => events,
        Err(e) => {
            error!("Failed to generate calendar data: {:?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error: {}", e),
            ));
        }
    };

    if events.is_empty() {
        info!("No structured events returned by AI");
        return Ok(Json(Vec::<Value>::new()));
    }

    Ok(Json(events))
}

#[derive(Serialize, Deserialize)]
pub struct MonthQuery {
    pub month: u32,
}
