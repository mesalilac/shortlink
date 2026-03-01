mod ping;
mod shorten;
mod url;

use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(ping::ping))
        .routes(routes!(shorten::shorten))
        .routes(routes!(url::get_url))
}
