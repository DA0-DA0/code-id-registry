#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw20::TokenInfoResponse;
use cw_utils::must_pay;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, GetRegistrationResponse, InstantiateMsg, ListRegistrationsResponse, QueryMsg,
};
use crate::state::{
    Registration, ADMIN, CHAIN_ID_CODE_ID_TO_REGISTRATION, NAME_CHAIN_ID_VERSION_TO_REGISTRATION,
};

const CONTRACT_NAME: &str = "crates.io:cw-code-id-registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let validated_admin = deps.api.addr_validate(&msg.admin)?;
    ADMIN.save(deps.storage, &validated_admin)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {
            contract_name: name,
            version,
            chain_id,
            code_id,
            checksum,
        } => execute_register(deps, info, name, version, chain_id, code_id, checksum),
        ExecuteMsg::Unregister {
            contract_name,
            chain_id,
            code_id,
            version,
        } => execute_unregister(deps, info.sender, contract_name, chain_id, code_id, version),
        ExecuteMsg::UpdateAdmin { admin } => execute_update_admin(deps, env, info, admin),
    }
}

/// (name, version, chain-id) --> code-id registrations may also be updated using this routine.
pub fn execute_register(
    deps: DepsMut,
    info: MessageInfo,
    contract_name: String,
    version: String,
    chain_id: String,
    code_id: u64,
    checksum: String,
) -> Result<Response, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::UnauthorizedRegistration {});
    }

    // Can't re-register a code ID on a chain.
    if CHAIN_ID_CODE_ID_TO_REGISTRATION
        .may_load(deps.storage, (&chain_id, code_id))?
        .is_some()
    {
        return Err(ContractError::CodeIDAlreadyRegistered(
            code_id,
            chain_id.clone(),
        ));
    }

    let registration = Registration {
        contract_name: contract_name.clone(),
        version: version.clone(),
        code_id,
        checksum: checksum.clone(),
    };

    // Add to state.
    CHAIN_ID_CODE_ID_TO_REGISTRATION.save(deps.storage, (&chain_id, code_id), &registration)?;
    NAME_CHAIN_ID_VERSION_TO_REGISTRATION.save(
        deps.storage,
        (&contract_name, &chain_id, &version),
        &registration,
    )?;

    Ok(Response::new()
        .add_attribute("action", "register_code_id")
        .add_attribute("code_id", code_id.to_string())
        .add_attribute("contract_name", contract_name))
}

pub fn execute_unregister(
    deps: DepsMut,
    sender: Addr,
    contract_name: String,
    chain_id: String,
    code_id: u64,
    version: String,
) -> Result<Response, ContractError> {
    let admin = ADMIN.load(deps.storage)?;

    // Only allow admin to unregister.
    if sender != admin {
        return Err(ContractError::UnauthorizedRegistration {});
    }

    // Remove registration.
    NAME_CHAIN_ID_VERSION_TO_REGISTRATION
        .remove(deps.storage, (&contract_name, &chain_id, &version));
    CHAIN_ID_CODE_ID_TO_REGISTRATION.remove(deps.storage, (&chain_id, code_id));

    Ok(Response::new()
        .add_attribute("action", "unregister")
        .add_attribute("chain_id", chain_id)
        .add_attribute("contract_name", contract_name)
        .add_attribute("code_id", code_id.to_string()))
}

pub fn execute_update_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    let current_admin = ADMIN.load(deps.storage)?;
    if info.sender != current_admin {
        return Err(ContractError::UnauthorizedUpdateAdmin {});
    }

    let validated_admin = deps.api.addr_validate(&new_admin)?;
    ADMIN.save(deps.storage, &validated_admin)?;

    Ok(Response::new()
        .add_attribute("action", "update_admin")
        .add_attribute("new_admin", new_admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admin {} => to_binary(&ADMIN.load(deps.storage)?),
        QueryMsg::GetRegistration {
            name,
            chain_id,
            version,
        } => query_get_registration(deps, name, chain_id, version),
        QueryMsg::GetCodeIdInfo { chain_id, code_id } => {
            query_get_code_id_info(deps, chain_id, code_id)
        }
        QueryMsg::ListRegistrations { name, chain_id } => {
            query_list_registrations(deps, name, chain_id)
        }
    }
}

pub fn query_get_registration(
    deps: Deps,
    contract_name: String,
    chain_id: String,
    version: Option<String>,
) -> StdResult<Binary> {
    let registration = match version {
        Some(version) => {
            let code_id = NAME_CHAIN_ID_VERSION_TO_REGISTRATION
                .load(deps.storage, (&contract_name, &chain_id, &version))
                .map_err(|_| StdError::GenericErr {
                    msg: ContractError::NotFound {}.to_string(),
                })?;

            let registration = NAME_CHAIN_ID_VERSION_TO_REGISTRATION
                .load(deps.storage, (&contract_name, &chain_id, &version))
                .map_err(|_| StdError::GenericErr {
                    msg: ContractError::NotFound {}.to_string(),
                })?;
            Ok::<Registration, StdError>(registration)
        }
        None => {
            let registration = NAME_CHAIN_ID_VERSION_TO_REGISTRATION
                .prefix((&contract_name, &chain_id))
                .range(deps.storage, None, None, Order::Descending)
                .next()
                .ok_or(StdError::GenericErr {
                    msg: ContractError::NotFound {}.to_string(),
                })?
                .map_err(|_| StdError::GenericErr {
                    msg: ContractError::NotFound {}.to_string(),
                })?;
            Ok(registration.1)
        }
    }?;

    to_binary(&GetRegistrationResponse { registration })
}

pub fn query_get_code_id_info(deps: Deps, chain_id: String, code_id: u64) -> StdResult<Binary> {
    // Retrieve registration.
    let registration = CHAIN_ID_CODE_ID_TO_REGISTRATION
        .load(deps.storage, (&chain_id, code_id))
        .map_err(|_| StdError::GenericErr {
            msg: ContractError::NotFound {}.to_string(),
        })?;

    to_binary(&GetRegistrationResponse { registration })
}

// TODO: Paginate.
pub fn query_list_registrations(deps: Deps, name: String, chain_id: String) -> StdResult<Binary> {
    let registrations = CHAIN_ID_CODE_ID_TO_REGISTRATION
        .prefix(&chain_id)
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<(u64, Registration)>>>()?
        .into_iter()
        .map(|(_, registration)| registration)
        .collect();
    to_binary(&ListRegistrationsResponse { registrations })
}
