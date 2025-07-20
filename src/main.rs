use axum::{
    routing::get,
    Router,
    response::Html,
};
use askama::Template;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::serve;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(homepage))
        .nest_service("/assets", ServeDir::new("public"));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}


async fn homepage() -> Html<String> {
    let html = IndexTemplate { title: "Marquez Heritage Society" }
        .render()
        .unwrap();
    Html(html)
}

