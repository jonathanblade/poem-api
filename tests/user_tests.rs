mod common;

use poem::http::StatusCode;
use serde_json::{json, Value as JsonValue};
use sqlx::SqlitePool;

#[sqlx::test(fixtures("user_table"))]
async fn get_users(pool: SqlitePool) {
    let mut client = common::TestClient::new(pool).await;

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

#[sqlx::test(fixtures("user_table"))]
async fn get_user(pool: SqlitePool) {
    let mut client = common::TestClient::new(pool).await;

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
