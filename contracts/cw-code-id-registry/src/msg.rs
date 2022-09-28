use crate::state::Registration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InstantiateMsg {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ExecuteMsg {
    /// Register code ID. May only be called by contract admin.
    Register {
        contract_name: String,
        version: String,
        chain_id: String,
        code_id: u64,
        checksum: String,
    },
    /// Allow admin to unregister code IDs.
    Unregister {
        contract_name: String,
        chain_id: String,
        code_id: u64,
        version: String,
    },
    /// Update admin.
    UpdateAdmin { admin: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum QueryMsg {
    Admin {},
    /// If version provided, tries to find given version. Otherwise returns
    /// the latest version registered.
    GetRegistration {
        name: String,
        chain_id: String,
        version: Option<String>,
    },
    GetCodeIdInfo {
        chain_id: String,
        code_id: u64,
    },
    ListRegistrations {
        name: String,
        chain_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct GetRegistrationResponse {
    pub registration: Registration,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct ListRegistrationsResponse {
    pub registrations: Vec<Registration>,
}
