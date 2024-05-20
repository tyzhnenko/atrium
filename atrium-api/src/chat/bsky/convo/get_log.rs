// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `chat.bsky.convo.getLog` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub logs: Vec<crate::types::Union<OutputLogsItem>>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum OutputLogsItem {
    #[serde(rename = "chat.bsky.convo.defs#logBeginConvo")]
    ChatBskyConvoDefsLogBeginConvo(Box<crate::chat::bsky::convo::defs::LogBeginConvo>),
    #[serde(rename = "chat.bsky.convo.defs#logLeaveConvo")]
    ChatBskyConvoDefsLogLeaveConvo(Box<crate::chat::bsky::convo::defs::LogLeaveConvo>),
    #[serde(rename = "chat.bsky.convo.defs#logCreateMessage")]
    ChatBskyConvoDefsLogCreateMessage(
        Box<crate::chat::bsky::convo::defs::LogCreateMessage>,
    ),
    #[serde(rename = "chat.bsky.convo.defs#logDeleteMessage")]
    ChatBskyConvoDefsLogDeleteMessage(
        Box<crate::chat::bsky::convo::defs::LogDeleteMessage>,
    ),
}