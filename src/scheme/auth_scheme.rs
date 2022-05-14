use poem_openapi::Object;

/// User credentials scheme.
#[derive(Object)]
#[oai(example = "credentials_example")]
pub struct Credentials {
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

/// Access token scheme.
#[derive(Object)]
#[oai(example = "access_token_example")]
pub struct AccessToken {
    /// Access token.
    pub token: String,
    /// Access token type.
    #[oai(default = "default_access_token_type")]
    pub token_type: String,
    /// Access token issuing time.
    pub issued_at: i64,
    /// Access token expiring time.
    pub expired_in: i64,
}

fn credentials_example() -> Credentials {
    Credentials {
        username: "admin".to_string(),
        password: "12345".to_string(),
    }
}

fn access_token_example() -> AccessToken {
    AccessToken {
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0MSIsInNjb3BlcyI6WyJhbGwiXSwiZXhwIjoxNjIzMjU1NTY3fQ.MJJe4b9UmpB7fiRasuv5dMESKyc6LJ-IQtt5X7nJ4bY".to_string(),
        token_type: "Bearer".to_string(),
        issued_at: 1651423450477,
        expired_in: 1651419850477,
    }
}

fn default_access_token_type() -> String {
    "Bearer".to_string()
}
