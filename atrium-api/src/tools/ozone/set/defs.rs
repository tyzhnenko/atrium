// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `tools.ozone.set.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
}
pub type Set = crate::types::Object<SetData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetViewData {
    pub created_at: crate::types::string::Datetime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
    pub set_size: i64,
    pub updated_at: crate::types::string::Datetime,
}
pub type SetView = crate::types::Object<SetViewData>;