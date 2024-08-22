#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, HexBinary, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:cw-clock-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            val: 0,
            hash: "".to_string(),
        },
    )?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

fn increment(deps: DepsMut, hash: String) -> Result<(), ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.val += 1;
    config.hash = HexBinary::from(Binary::from_base64(&hash)?).to_hex();
    CONFIG.save(deps.storage, &config)?;
    Ok(())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

// sudo msg
#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ClockEndBlock { hash } => {
            increment(deps, hash)?;
            Ok(Response::new())
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let count = CONFIG.load(deps.storage)?;
    Ok(count)
}
