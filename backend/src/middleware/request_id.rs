use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    req.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&id).unwrap(),
    );

    let mut response = next.run(req).await;
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&id).unwrap(),
    );
    response
}
