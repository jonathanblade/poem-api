mod common;

use poem::http::StatusCode;
use serde_json::{json, Value as JsonValue};

#[tokio::test]
async fn sign_in() {
    let client = common::TestClient::new().await;

    let resp = client
        .post(
            "/api/auth/token",
            &json!({"username": "admin", "password": "12345"}),
        )
        .send()
        .await;
    resp.assert_status(StatusCode::CREATED);

    let resp = client
        .post(
            "/api/auth/token",
            &json!({"username": "foo", "password": "foo"}),
        )
        .send()
        .await;
    resp.assert_status(StatusCode::UNAUTHORIZED);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(
        resp,
        json!({
            "status": "error",
            "reason": "Invalid credentials."
        })
    )
}
