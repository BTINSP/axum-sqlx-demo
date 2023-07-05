use axum::{http::{Request}, middleware::Next, response::Response};






pub async fn authenticator<B>(request: Request<B>, next: Next<B>, ) -> Response {
    let uri = request.uri().path();
    println!("{}",uri);
    next.run(request).await
}