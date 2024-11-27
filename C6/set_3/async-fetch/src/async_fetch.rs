use std::time::Duration;

use url::Url;

pub async fn async_fetch(url: &str) -> Result<String, String> {
    Url::parse(url).map_err(|e| format!("invalid url, error: {}", e))?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    Ok("API_RESPONSE".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn valid_url() {
        assert_eq!(
            async_fetch("https://localhost:3000").await,
            Ok("API_RESPONSE".into())
        );
    }

    #[tokio::test]
    async fn invalid_url() {
        assert!(async_fetch("fdsjfsfj09sj").await.is_err());
    }
}
