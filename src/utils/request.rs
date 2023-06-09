use reqwest::{Client, Response};
use serde::Serialize;

const BASE: &str = "http://localhost:8000";

/// Convenience method to send a get request
pub async fn get(url: &str, token: String) -> Result<Response, reqwest::Error> {
    Client::new()
        .get(format!("{BASE}{url}"))
        .header("x-token", token)
        .send()
        .await
}

/// Convenience method to send a post request
pub async fn post<T>(url: &str, json: &T, token: String) -> Result<Response, reqwest::Error>
where
    T: Serialize + ?Sized,
{
    Client::new()
        .post(format!("{BASE}{url}"))
        .header("x-token", token)
        .json(json)
        .send()
        .await
}

/// Convenience method to send a put request
pub async fn put<T>(url: &str, json: &T, token: String) -> Result<Response, reqwest::Error>
where
    T: Serialize + ?Sized,
{
    Client::new()
        .put(format!("{BASE}{url}"))
        .header("x-token", token)
        .json(json)
        .send()
        .await
}

/// Convenience method to send a delete request
pub async fn delete(url: &str, token: String) -> Result<Response, reqwest::Error> {
    Client::new()
        .delete(format!("{BASE}{url}"))
        .header("x-token", token)
        .send()
        .await
}
