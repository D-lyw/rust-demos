mod urls;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq)]
pub(crate) struct Url {
    pub id: i32,
    pub url: String,
    pub short_url: String,
}
