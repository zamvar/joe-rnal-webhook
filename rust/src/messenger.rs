//! Google Chat Messenger integration with environment caching and URL safety validation.

use reqwest::{Client, Response, Url};
use std::collections::HashMap;
use std::sync::OnceLock;
use rand::seq::SliceRandom;
use std::time::Duration;

/// Global thread-safe cache for parsed user IDs.
static CACHED_MENTIONS: OnceLock<Vec<String>> = OnceLock::new();

/// Parses and caches environment variable `MENTIONS_HYDRATION` (falling back to `MENTIONS`) user IDs.
/// Split by commas, trimmed, and empty IDs filtered out.
/// Returns a reference to the cached slice of user IDs.
pub fn get_cached_mentions() -> &'static [String] {
    CACHED_MENTIONS.get_or_init(|| {
        let list = std::env::var("MENTIONS_HYDRATION")
            .or_else(|_| std::env::var("MENTIONS"))
            .unwrap_or_default();
        
        if list.trim().is_empty() {
            Vec::new()
        } else {
            list.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    })
}

/// Retrieves hydration mention tags.
/// If there are cached user IDs, one is selected at random and formatted as:
/// `<users/all> (lalo na kay <users/{randomUser}>)`.
/// Otherwise, defaults to `<users/all>`.
pub fn get_hydration_mentions() -> String {
    let mentions = get_cached_mentions();
    if mentions.is_empty() {
        "<users/all>".to_string()
    } else {
        let mut rng = rand::thread_rng();
        if let Some(random_user) = mentions.choose(&mut rng) {
            format!("<users/all> (lalo na kay <users/{}>)", random_user)
        } else {
            "<users/all>".to_string()
        }
    }
}

/// Validates the Google Chat Webhook URL scheme and host.
/// Rejects any URL that does not use HTTPS or whose host is not 'chat.googleapis.com'.
/// This prevents Server-Side Request Forgery (SSRF) and request routing to malicious targets.
pub fn validate_url(url_str: &str) -> Result<Url, String> {
    let url = Url::parse(url_str).map_err(|e| format!("Invalid URL structure: {}", e))?;
    
    if url.scheme() != "https" {
        return Err("Insecure URL scheme: only HTTPS is allowed".to_string());
    }
    
    let host = url.host_str().ok_or_else(|| "Missing host in URL".to_string())?;
    if host.to_lowercase() != "chat.googleapis.com" {
        return Err(format!(
            "Unauthorized webhook host '{}': Only 'chat.googleapis.com' is allowed to prevent SSRF.",
            host
        ));
    }
    
    Ok(url)
}

/// Helper function to sanitize sensitive URL webhook tokens from error strings
pub fn sanitize_error(err_str: &str) -> String {
    let re = regex::Regex::new(r"https://chat\.googleapis\.com/[^\s)'&quot;\]}]*").unwrap();
    re.replace_all(err_str, "https://chat.googleapis.com/[REDACTED]").into_owned()
}

/// Sends an asynchronous POST message to Google Chat webhook URL with the given text payload.
/// Performs safety checks on the URL (HTTPS-only, host chat.googleapis.com).
pub async fn send_message(url_str: &str, text: &str) -> Result<Response, String> {
    let validated_url = validate_url(url_str)?;
    
    let mut payload = HashMap::new();
    payload.insert("text", text);
    
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create reqwest client: {}", e))?;

    let response = client
        .post(validated_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            let raw_err = format!("{}", e);
            sanitize_error(&raw_err)
        })?;
        
    Ok(response)
}
