// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `chat.bsky.convo.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConvoViewData {
    pub id: String,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub last_message: core::option::Option<
        crate::types::Union<ConvoViewLastMessageRefs>,
    >,
    pub members: Vec<crate::chat::bsky::actor::defs::ProfileViewBasic>,
    pub muted: bool,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub opened: core::option::Option<bool>,
    pub rev: String,
    pub unread_count: i64,
}
pub type ConvoView = crate::types::Object<ConvoViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeletedMessageViewData {
    pub id: String,
    pub rev: String,
    pub sender: MessageViewSender,
    pub sent_at: crate::types::string::Datetime,
}
pub type DeletedMessageView = crate::types::Object<DeletedMessageViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogBeginConvoData {
    pub convo_id: String,
    pub rev: String,
}
pub type LogBeginConvo = crate::types::Object<LogBeginConvoData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogCreateMessageData {
    pub convo_id: String,
    pub message: crate::types::Union<LogCreateMessageMessageRefs>,
    pub rev: String,
}
pub type LogCreateMessage = crate::types::Object<LogCreateMessageData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogDeleteMessageData {
    pub convo_id: String,
    pub message: crate::types::Union<LogDeleteMessageMessageRefs>,
    pub rev: String,
}
pub type LogDeleteMessage = crate::types::Object<LogDeleteMessageData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogLeaveConvoData {
    pub convo_id: String,
    pub rev: String,
}
pub type LogLeaveConvo = crate::types::Object<LogLeaveConvoData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageInputData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub embed: core::option::Option<crate::types::Union<MessageInputEmbedRefs>>,
    ///Annotations of text (mentions, URLs, hashtags, etc)
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub facets: core::option::Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    pub text: String,
}
pub type MessageInput = crate::types::Object<MessageInputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageRefData {
    pub convo_id: String,
    pub did: crate::types::string::Did,
    pub message_id: String,
}
pub type MessageRef = crate::types::Object<MessageRefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageViewData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub embed: core::option::Option<crate::types::Union<MessageViewEmbedRefs>>,
    ///Annotations of text (mentions, URLs, hashtags, etc)
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub facets: core::option::Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    pub id: String,
    pub rev: String,
    pub sender: MessageViewSender,
    pub sent_at: crate::types::string::Datetime,
    pub text: String,
}
pub type MessageView = crate::types::Object<MessageViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageViewSenderData {
    pub did: crate::types::string::Did,
}
pub type MessageViewSender = crate::types::Object<MessageViewSenderData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ConvoViewLastMessageRefs {
    #[serde(rename = "chat.bsky.convo.defs#messageView")]
    MessageView(Box<MessageView>),
    #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
    DeletedMessageView(Box<DeletedMessageView>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum LogCreateMessageMessageRefs {
    #[serde(rename = "chat.bsky.convo.defs#messageView")]
    MessageView(Box<MessageView>),
    #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
    DeletedMessageView(Box<DeletedMessageView>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum LogDeleteMessageMessageRefs {
    #[serde(rename = "chat.bsky.convo.defs#messageView")]
    MessageView(Box<MessageView>),
    #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
    DeletedMessageView(Box<DeletedMessageView>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum MessageInputEmbedRefs {
    #[serde(rename = "app.bsky.embed.record")]
    AppBskyEmbedRecordMain(Box<crate::app::bsky::embed::record::Main>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum MessageViewEmbedRefs {
    #[serde(rename = "app.bsky.embed.record#view")]
    AppBskyEmbedRecordView(Box<crate::app::bsky::embed::record::View>),
}
