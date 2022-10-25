use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let pool = app::db::prepare_db(&format!(
        "sqlite://{}",
        std::env::var("APP_DB_FILE").unwrap_or("poem-example-app.db".to_string())
    ))
    .await;
    let app = app::create_app(pool).await;
    let listener = TcpListener::bind(&addr);
    Server::new(listener).run(app).await
}
