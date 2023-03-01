pub mod common;
pub mod context;
pub mod controller;
pub mod db;
pub mod middleware;
pub mod response;
pub mod scheme;
pub mod service;

use poem::{
    middleware::{AddData, AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sqlx::SqlitePool;

pub type App =
    AddDataEndpoint<CorsEndpoint<middleware::ErrorMiddlewareImpl<Route>>, context::AppContext>;

// -> impl Endpoint
pub async fn create_app(pool: SqlitePool) -> App {
    let ctx = context::AppContext::new(pool);
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
        .with(AddData::new(ctx));
    app
}
