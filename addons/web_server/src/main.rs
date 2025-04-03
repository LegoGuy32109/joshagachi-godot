use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::{response::Response, Router};
use tower_http::services::ServeDir;

async fn add_cors_headers(req: Request, next: Next) -> Result<Response, Response> {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert(
        "Cross-Origin-Embedder-Policy",
        "require-corp".parse().unwrap(),
    );
    headers.insert("Cross-Origin-Opener-Policy", "same-origin".parse().unwrap());
    Ok(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("../../exports"))
        .layer(middleware::from_fn(add_cors_headers));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8061")
        .await
        .unwrap();

    println!(
        "\nlistening on [[ http://{} ]] <| Shift+Click Me!",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
