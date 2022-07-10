use axum::{routing, Router};

use crate::infra::handler;
use std::sync::Arc;

pub struct Di {
    pub handler: Arc<handler::Handler>,
}

pub fn new() -> Di {
    let h = Arc::new(handler::Handler {});
    Di {
        handler: h,
    }
}

impl Di {
    pub fn router(&self) -> Router {
        let h = self.handler.clone();
        Router::new().route("/", routing::get(|| async move { h.handle().await }))
    }
}
