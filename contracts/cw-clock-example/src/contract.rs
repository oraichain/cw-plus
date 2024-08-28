#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, HexBinary, MessageInfo, Response,
    StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, AFTER_SUDO, CONFIG};

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
    AFTER_SUDO.save(deps.storage, &0)?;

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
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AfterSudo {} => {
            let mut config = AFTER_SUDO.load(deps.storage)?;
            config += 5;
            AFTER_SUDO.save(deps.storage, &config)?;
            Ok(Response::new())
        }
    }
}

// sudo msg
#[entry_point]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ClockEndBlock { hash } => {
            increment(deps, hash)?;
            Ok(
                Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: env.contract.address.into_string(),
                    msg: to_json_binary(&ExecuteMsg::AfterSudo {})?,
                    funds: vec![],
                })),
            )
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetAfterSudo {} => to_json_binary(&AFTER_SUDO.load(deps.storage)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let count = CONFIG.load(deps.storage)?;
    Ok(count)
}
