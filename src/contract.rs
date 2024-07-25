use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    StdError,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub message: String,
}

pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub message: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State { message: msg.message };
    STATE.save(deps.storage, &state)?;
    set_contract_version(deps.storage, "hello_world", "1.0.0")?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: Binary) -> StdResult<Binary> {
    match STATE.load(_deps.storage) {
        Ok(state) => to_json_binary(&state.message),
        Err(_) => Err(StdError::not_found("state")),
    }
}

// --- Unit Tests ---

#[cfg(test)] // Only compile this module during tests
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::from_json;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { message: "Hello".to_string() };
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        
        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), Binary::default()).unwrap();
        let value: String = from_json(&res).unwrap();
        assert_eq!("Hello", value);
    }
}
