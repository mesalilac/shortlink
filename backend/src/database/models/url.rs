use crate::schema;
use diesel::prelude::*;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::time;

#[derive(
    Queryable, Serialize, Deserialize, Identifiable, Insertable, Debug, Clone, utoipa::ToSchema,
)]
#[diesel(table_name = schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub id: String,
    pub long_url: String,
    pub clicks: i32,
    pub expires_at: Option<i64>,
    pub max_clicks: Option<i32>,
    pub disabled: bool,
    pub last_clicked_at: Option<i64>,
    pub created_at: i64,
}

impl Url {
    pub fn new(long_url: String, expires_at: Option<i64>, max_clicks: Option<i32>) -> Self {
        let Ok(created_at) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
            tracing::error!("Failed to get current time");
            std::process::exit(1);
        };

        Self {
            id: nanoid!(),
            long_url,
            clicks: 0,
            expires_at,
            max_clicks,
            disabled: false,
            last_clicked_at: None,
            created_at: created_at.as_millis() as i64,
        }
    }
}
