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

fn strip_markdown_wrappers(s: &str) -> &str {
    s.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

async fn events_handler(Query(params): Query<MonthQuery>) ->  Result<impl IntoResponse, impl IntoResponse> {
    info!("Fetching structured events for month: {}", params.month);

    let raw = match generate_structured_calendar_data(params.month).await {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to generate calendar data: {:?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error: {}", e),
            ));
        }
    };

    if raw.is_empty() {
        info!("No structured events returned by AI");
        return Ok(Json(Vec::<Value>::new()));
    }

    let mut parsed = Vec::new();
    for (i, json_str) in raw.iter().enumerate() {
        let cleaned = strip_markdown_wrappers(json_str);
        match serde_json::from_str::<Value>(cleaned) {
            Ok(val) => parsed.push(val),
            Err(e) => {
                error!("Failed to parse event #{}: {}", i, e);
                error!("Raw string: {}", json_str);
            }
        }
    }

    Ok(Json(parsed))
}

#[derive(Serialize, Deserialize)]
pub struct MonthQuery {
    pub month: u32,
}
