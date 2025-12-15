use axum::http::StatusCode;

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("Unhandled internal error: {:#?}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong".to_string(),
    )
}
