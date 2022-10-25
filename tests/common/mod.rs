use poem::{
    http::header::AUTHORIZATION,
    test::{TestClient as PoemTestClient, TestRequestBuilder},
};
use serde_json::{json, Value as JsonValue};
use sqlx::SqlitePool;

pub struct TestClient {
    client: PoemTestClient<app::App>,
    token: Option<String>,
}

impl TestClient {
    pub async fn new(pool: SqlitePool) -> TestClient {
        let app = app::create_app(pool).await;
        let client = PoemTestClient::new(app);
        TestClient {
            client,
            token: None,
        }
    }

    pub async fn sign_in(&mut self, username: &str, password: &str) {
        let resp = self
            .client
            .post("/api/auth/token")
            .body_json(&json!({"username": username, "password": password}))
            .send()
            .await;
        let resp: JsonValue = resp.json().await.value().deserialize();
        self.token = Some(resp["result"]["token"].as_str().unwrap().to_string());
    }

    pub fn get(&self, uri: &str) -> TestRequestBuilder<app::App> {
        let req = self.client.get(uri);
        if self.token.is_some() {
            req.header(
                AUTHORIZATION,
                format!("Bearer {}", self.token.clone().unwrap()),
            )
        } else {
            req
        }
    }

    pub fn post(&self, uri: &str, body: &JsonValue) -> TestRequestBuilder<app::App> {
        let req = self.client.post(uri).body_json(body);
        if self.token.is_some() {
            req.header(
                AUTHORIZATION,
                format!("Bearer {}", self.token.clone().unwrap()),
            )
        } else {
            req
        }
    }
}
