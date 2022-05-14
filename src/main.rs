use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let app = app::create_app(true).await;
    let listener = TcpListener::bind(&addr);
    Server::new(listener).run(app).await
}
