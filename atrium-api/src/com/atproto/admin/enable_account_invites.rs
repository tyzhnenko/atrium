// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.admin.enableAccountInvites` namespace."]
#[doc = "`com.atproto.admin.enableAccountInvites`"]
#[doc = "Re-enable an accounts ability to receive invite codes"]
#[async_trait::async_trait]
pub trait EnableAccountInvites: crate::xrpc::XrpcClient {
    async fn enable_account_invites(&self, input: Input) -> Result<(), Box<dyn std::error::Error>> {
        let _ = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::POST,
            "com.atproto.admin.enableAccountInvites",
            None,
            Some(serde_json::to_vec(&input)?),
            Some(String::from("application/json")),
        )
        .await?;
        Ok(())
    }
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub account: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}