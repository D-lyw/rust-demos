use sqlx::PgPool;

use crate::error::AppError;

use super::Url;

#[allow_dead_code]
pub(crate) struct CreateUrl {
    pub url: String,
    pub short_url: String,
}

impl Url {
    pub async fn create(
        url: &str,
        shorten_url: &str,
        pool: &PgPool,
    ) -> Result<CreateUrl, AppError> {
        // check if url already exists
        let url_exists = sqlx::query!(
            "SELECT EXISTS (SELECT 1 FROM url_shortener_service WHERE url = $1)",
            url
        )
        .fetch_one(pool)
        .await?.exists;

        if let Some(true) = url_exists {
            return Err(AppError::AlreadyExists(url.into()));
        }

        // insert new record into database
        // and return shortened url and id
        let _item = sqlx::query_as(r#"INSERT INTO url_shortener_service (url, short_url) VALUES ($1, $2) RETURNING id, url, short_url"#).bind(url).bind(shorten_url).fetch_one(pool).await?;

        Ok(CreateUrl {
            url: url.into(),
            short_url: shorten_url.into(),
        })
    }

    pub async fn find_url(shorten_url: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let item = sqlx::query_as(
            r#"SELECT id, url, short_url FROM url_shortener_service WHERE short_url = $1"#,
        )
        .bind(shorten_url)
        .fetch_optional(pool)
        .await?;

        Ok(item)
    }
}
