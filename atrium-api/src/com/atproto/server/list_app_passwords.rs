// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.server.listAppPasswords` namespace.

/// List all app-specific passwords.
#[async_trait::async_trait]
pub trait ListAppPasswords: crate::xrpc::XrpcClient {
    async fn list_app_passwords(&self) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.server.listAppPasswords",
            Option::<()>::None,
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub passwords: Vec<AppPassword>,
}

pub enum Error {
    AccountTakedown,
}

// com.atproto.server.listAppPasswords#appPassword
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppPassword {
    pub created_at: String,
    pub name: String,
}