use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Registration {
    pub contract_name: String,
    pub version: String,
    pub code_id: u64,
    pub checksum: String,
}

/// The admin has sole permissions to register code IDs.
pub const ADMIN: Item<Addr> = Item::new("admin");
/// Map (name, chain_id, version) to a code_id.
pub const NAME_CHAIN_ID_VERSION_TO_REGISTRATION: Map<(&str, &str, &str), Registration> =
    Map::new("name_chain_id_version_to_code_id");
/// Map (name, chain_id, code_id) to the registration.
pub const CHAIN_ID_CODE_ID_TO_REGISTRATION: Map<(&str, u64), Registration> =
    Map::new("chain_id_code_id_to_registration");
