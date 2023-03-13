#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{
        app,
        handlers::post::{CreatePostResponse, CreatePostRouteParams, Post},
    };

    fn create_post_request() -> CreatePostRouteParams {
        CreatePostRouteParams {
            title: "test title".to_string(),
            body: "test body".to_string(),
            userId: 1,
        }
    }

    fn create_post_response() -> CreatePostResponse {
        CreatePostResponse { id: 101 }
    }

    fn get_post_response() -> Post {
        Post {
            id: 1,
            title: "sunt aut facere repellat provident occaecati excepturi optio reprehenderit".to_string(),
            body: "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto".to_string(),
            userId: 1
        }
    }

    #[tokio::test]
    async fn test_create_posts() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/posts")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&create_post_request()).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: CreatePostResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, create_post_response());
    }

    #[tokio::test]
    async fn test_get_posts() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/posts/1")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Post = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, get_post_response());
    }
}
