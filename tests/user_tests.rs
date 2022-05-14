mod common;

use poem::http::StatusCode;
use serde_json::{json, Value as JsonValue};

#[tokio::test]
async fn get_users() {
    let mut client = common::TestClient::new().await;

    client.sign_in("admin", "12345").await;

    let resp = client.get("/api/user").send().await;
    resp.assert_status(StatusCode::OK);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(
        resp,
        json!({
            "status": "success",
            "result": {
                "items": [
                    {
                        "id": 1,
                        "username": "admin",
                        "is_superuser": 1
                    },
                    {
                        "id": 2,
                        "username": "manager",
                        "is_superuser": 0
                    }
                ],
                "offset": 0,
                "total": 2
            }
        })
    );
}

#[tokio::test]
async fn get_user() {
    let mut client = common::TestClient::new().await;

    client.sign_in("admin", "12345").await;

    let resp = client.get("/api/user/1").send().await;
    resp.assert_status(StatusCode::OK);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(
        resp,
        json!({
            "status": "success",
            "result": {
                "id": 1,
                "username": "admin",
                "is_superuser": 1
            }
        })
    );

    let resp = client.get("/api/user/3").send().await;
    resp.assert_status(StatusCode::NOT_FOUND);
}
