use axum::{http::{Request}, middleware::Next, response::Response};


pub async fn authenticator<B>(request: Request<B>, next: Next<B>, ) -> Response {
    let path = request.uri().path();
    println!("{}", path);
    let response = next.run(request).await;
    response
}