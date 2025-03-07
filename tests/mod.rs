use axum::response::IntoResponse;
use into_response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Serialize, IntoResponse)]
struct MyResponse {
    message: String,
}

#[derive(Debug, Serialize, IntoResponse)]
#[into_response(status = 201)]
struct MyResponse2 {
    message: String,
}

#[derive(Debug, Serialize, IntoResponse)]
#[into_response(status = 404)]
struct MyResponse3 {
    message: String,
}

#[test]
fn test_into_response() {
    let response = MyResponse {
        message: "Hello, world!".to_string(),
    };

    let response = response.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let response2 = MyResponse2 {
        message: "Hello, world!".to_string(),
    };

    let response2 = response2.into_response();
    assert_eq!(response2.status(), axum::http::StatusCode::CREATED);

    let response3 = MyResponse3 {
        message: "Hello, world!".to_string(),
    };

    let response3 = response3.into_response();
    assert_eq!(response3.status(), axum::http::StatusCode::NOT_FOUND);
}
