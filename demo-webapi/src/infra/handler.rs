use axum::response::Json;
use serde_json::{json, Value};

pub struct Handler {}

impl Handler {
    pub async fn handle(&self) -> Json<Value> {
        Json(json!({"data": 42}))
    }
}
