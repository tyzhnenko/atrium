// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.temp.transferAccount` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub did: crate::types::string::Did,
    pub handle: crate::types::string::Handle,
    pub plc_op: crate::records::Record,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub access_jwt: String,
    pub did: crate::types::string::Did,
    pub handle: crate::types::string::Handle,
    pub refresh_jwt: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    InvalidHandle(Option<String>),
    InvalidPassword(Option<String>),
    InvalidInviteCode(Option<String>),
    HandleNotAvailable(Option<String>),
    UnsupportedDomain(Option<String>),
    UnresolvableDid(Option<String>),
    IncompatibleDidDoc(Option<String>),
}