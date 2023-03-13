use axum::{extract::Path, response::IntoResponse, Json};
use http::StatusCode;
use serde_json::{from_str, json};
use validator::Validate;

use crate::{config::POSTS_URL, errors::ApiError, validate::validate};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub userId: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Validate, PartialEq, Eq)]
pub struct CreatePostRouteParams {
    #[validate(length(
        min = 2,
        message = "Title is required and must be at least 2 characters"
    ))]
    pub title: String,
    pub body: String,
    pub userId: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreatePostResponse {
    pub id: u64,
}

/// POST "/posts"
/// Creates a post
pub async fn create_post_endpoint(
    Json(payload): Json<CreatePostRouteParams>,
) -> Result<impl IntoResponse, ApiError> {
    validate(&payload)?;
    let client = reqwest::Client::new(); // TODO: move to app state?
    let response = client
        .post(POSTS_URL.clone())
        .body(json!(payload).to_string())
        .send()
        .await?;

    let post: CreatePostResponse = from_str(&response.text().await?)?;

    return Ok((StatusCode::OK, Json(post)));
}

/// GET "/posts/:id"
/// Gets a post
pub async fn get_post_endpoint(Path(id): Path<u64>) -> Result<impl IntoResponse, ApiError> {
    let url = format!("{}/{}", POSTS_URL.clone(), id);
    let response = reqwest::get(url).await?.text().await?;

    let post: Post = from_str(&response)?;
    Ok((StatusCode::OK, Json(post)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_post() {
        let response = get_post_endpoint(Path(1)).await.unwrap().into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_post() {
        let request = CreatePostRouteParams {
            title: "Test Title".to_string(),
            body: "Test Body".to_string(),
            userId: 5,
        };

        let response = create_post_endpoint(Json(request))
            .await
            .unwrap()
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
