// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `tools.ozone.signature.findRelatedAccounts` namespace.
pub const NSID: &str = "tools.ozone.signature.findRelatedAccounts";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub did: crate::types::string::Did,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub accounts: Vec<RelatedAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
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
pub struct RelatedAccountData {
    pub account: crate::com::atproto::admin::defs::AccountView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarities: Option<Vec<crate::tools::ozone::signature::defs::SigDetail>>,
}
pub type RelatedAccount = crate::types::Object<RelatedAccountData>;