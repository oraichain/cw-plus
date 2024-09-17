#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, to_json_vec, Binary, Deps, DepsMut, Empty, Env, MessageInfo, QuerierWrapper,
    QueryRequest, Response, StdResult,
};
use cw2::set_contract_version;
use ibc_proto::cosmos::staking::v1beta1::{QueryValidatorRequest, QueryValidatorResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use prost::Message;

const CONTRACT_NAME: &str = "crates.io:cw-stargate-staking-query-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::StakingValidator { val_addr } => {
            to_json_binary(&query_staking_validator(deps.querier, val_addr)?)
        }
    }
}

pub fn query_staking_validator(api: QuerierWrapper, addr: String) -> StdResult<String> {
    let bin_request = to_json_vec(&QueryRequest::<Empty>::Stargate {
        path: "/cosmos.staking.v1beta1.Query/Validator".to_string(),
        data: QueryValidatorRequest {
            validator_addr: addr,
        }
        .encode_to_vec()
        .into(),
    })?;
    let buf = api.raw_query(&bin_request).unwrap().unwrap();
    let validator_response = QueryValidatorResponse::decode(buf.as_slice()).unwrap();
    let validator = validator_response.validator.unwrap();
    Ok(validator.operator_address)
}
