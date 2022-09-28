pub mod common;
pub mod controller;
pub mod db;
pub mod middleware;
pub mod response;
pub mod scheme;
pub mod service;

use poem::{
    middleware::{AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sqlx::SqlitePool;

pub type App = AddDataEndpoint<CorsEndpoint<middleware::ErrorMiddlewareImpl<Route>>, SqlitePool>;

// -> impl Endpoint
pub async fn create_app(test_mode: bool) -> App {
    let pool = db::utils::prepare_db(
        &format!(
            "sqlite://{}",
            std::env::var("APP_DB_FILE").unwrap_or("poem-example-app.db".to_string())
        ),
        test_mode,
    )
    .await;
    let api = OpenApiService::new(
        (controller::AuthController, controller::UserController),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .server("https://poem-example-app.herokuapp.com/api");
    let ui = api.swagger_ui();
    let app = Route::new()
        .nest("/api", api)
        .nest("/", ui)
        .with(middleware::ErrorMiddleware)
        .with(Cors::new())
        .data(pool);
    app
}
