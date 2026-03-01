use crate::schema::urls;
use crate::{database::models::url::Url, state::AppState};
use axum::extract::{Json, State};
use diesel::dsl::insert_into;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateShortUrlRequest {
    pub url: String,
    pub expires_at: Option<i64>,
    pub max_clicks: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateShortUrlResponse {
    pub url: String,
}

/// Creates a short url
///
/// Creates a short url from a long url
#[utoipa::path(
    post,
    path = "/shorten",
    request_body = CreateShortUrlRequest,
    tag = "Url",
    responses(
        (status = OK, description = "Created short url", body = CreateShortUrlResponse)
    )
)]
pub async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<CreateShortUrlRequest>,
) -> Result<Json<CreateShortUrlResponse>, String> {
    let mut conn = state
        .pool
        .get()
        .map_err(|_| "Could not get database connection")?;

    let base_url =
        std::env::var("BASE_URL").map_err(|_| "Missing `BASE_URL` environment variable")?;
    let new_url = Url::new(payload.url, payload.expires_at, payload.max_clicks);

    match insert_into(urls::table).values(&new_url).execute(&mut conn) {
        Ok(_) => Ok(Json(CreateShortUrlResponse {
            url: format!("{}/{}", base_url, new_url.id),
        })),
        Err(e) => Err(e.to_string()),
    }
}
