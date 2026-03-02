use std::time;

use crate::schema::urls;
use crate::{database::models::url::Url, state::AppState};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use diesel::dsl::update;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

fn disable_url(conn: &mut PgConnection, id: &str) -> Result<(), (StatusCode, String)> {
    update(urls::table.find(&id))
        .set(urls::disabled.eq(true))
        .execute(conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to disable url".into(),
            )
        })?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUrlResponse {
    pub long_url: String,
}

/// Redirects to the long url
///
/// Redirects to the long url
#[utoipa::path(get, path = "/url/{id}", tag = "Url", responses(
    (status = StatusCode::OK, description = "Redirects to the long url", body = GetUrlResponse),
    (status = StatusCode::FORBIDDEN, description = "Url is disabled", body = ()),
    (status = StatusCode::NOT_FOUND, description = "Url not found", body = ()),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Failed to get database connection", body = ()),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Failed to update database row", body = ()),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Failed to get current time", body = ()),
))]
pub async fn get_url(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetUrlResponse>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not get database connection".into(),
        )
    })?;

    let current_time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get current time".into(),
            )
        })?;

    let timestamp = current_time.as_millis() as i64;

    let url = urls::table
        .find(&id)
        .get_result::<Url>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Url not found".into()))?;

    if url.disabled {
        return Err((StatusCode::FORBIDDEN, "Url is disabled".into()));
    }

    if let Some(expires_at) = url.expires_at
        && expires_at < timestamp
    {
        disable_url(&mut conn, &id)?;

        return Err((StatusCode::FORBIDDEN, "Url has expired".into()));
    }

    if let Some(max_clicks) = url.max_clicks
        && url.clicks >= max_clicks
    {
        disable_url(&mut conn, &id)?;

        return Err((
            StatusCode::FORBIDDEN,
            "Url has reached its maximum number of allowed clicks".into(),
        ));
    }

    update(urls::table.find(&id))
        .set(urls::clicks.eq(url.clicks + 1))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update url clicks".into(),
            )
        })?;

    update(urls::table.find(&id))
        .set(urls::last_clicked_at.eq(timestamp))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update url clicks".into(),
            )
        })?;

    Ok(Json(GetUrlResponse {
        long_url: url.long_url,
    }))
}

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUrlInfoResponse {
    pub id: String,
    pub short_url: String,
    pub long_url: String,
    pub clicks: i32,
    pub expires_at: Option<i64>,
    pub max_clicks: Option<i32>,
    pub disabled: bool,
    pub last_clicked_at: Option<i64>,
    pub created_at: i64,
}

// Returns short url info and stats
///
/// Returns short url info and stats
#[utoipa::path(get, path = "/url/{id}/info", tag = "Url", responses(
    (status = StatusCode::OK, description = "url info", body = GetUrlInfoResponse),
    (status = StatusCode::NOT_FOUND, description = "Url not found", body = String),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Something went wrong", body = String),
))]
pub async fn get_url_info(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetUrlInfoResponse>, (StatusCode, String)> {
    let base_url = std::env::var("BASE_URL").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Missing `BASE_URL` environment variable".into(),
        )
    })?;

    let mut conn = state.pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not get database connection".into(),
        )
    })?;

    let url = urls::table
        .find(&id)
        .get_result::<Url>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Url not found".into()))?;

    Ok(Json(GetUrlInfoResponse {
        short_url: format!("{}/{}", base_url, url.id),
        id: url.id,
        long_url: url.long_url,
        clicks: url.clicks,
        expires_at: url.expires_at,
        max_clicks: url.max_clicks,
        disabled: url.disabled,
        last_clicked_at: url.last_clicked_at,
        created_at: url.created_at,
    }))
}
