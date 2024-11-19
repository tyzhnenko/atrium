// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.describeFeedGenerator` namespace.
pub const NSID: &str = "app.bsky.feed.describeFeedGenerator";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub did: crate::types::string::Did,
    pub feeds: Vec<Feed>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub links: core::option::Option<Links>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedData {
    pub uri: String,
}
pub type Feed = crate::types::Object<FeedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LinksData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub privacy_policy: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub terms_of_service: core::option::Option<String>,
}
pub type Links = crate::types::Object<LinksData>;
