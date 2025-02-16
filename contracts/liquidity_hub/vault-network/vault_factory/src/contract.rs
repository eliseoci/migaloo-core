use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

use vault_network::vault_factory::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::err::{StdResult, VaultFactoryError};
use crate::execute::{create_vault, migrate_vaults, update_config, update_vault_config};
use crate::queries::{get_config, get_vault, get_vaults};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "vault_factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        vault_id: msg.vault_id,
        token_id: msg.token_id,
        fee_collector_addr: deps.api.addr_validate(&msg.fee_collector_addr)?,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateVault { asset_info, fees } => {
            create_vault(deps, env, info, asset_info, fees)
        }
        ExecuteMsg::UpdateVaultConfig { vault_addr, params } => {
            update_vault_config(deps, info, vault_addr, params)
        }
        ExecuteMsg::MigrateVaults {
            vault_addr,
            vault_code_id,
        } => migrate_vaults(deps, info, vault_addr, vault_code_id),
        ExecuteMsg::UpdateConfig {
            owner,
            fee_collector_addr,
            vault_id,
            token_id,
        } => update_config(deps, info, owner, fee_collector_addr, vault_id, token_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

    if storage_version > version {
        return Err(VaultFactoryError::MigrateInvalidVersion {
            current_version: storage_version,
            new_version: version,
        });
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Response;

    use vault_network::vault_factory::MigrateMsg;

    use crate::tests::mock_instantiate::mock_instantiate;

    use super::migrate;

    #[test]
    fn can_migrate() {
        // instantiate contract
        let (mut deps, env) = mock_instantiate(5, 6);

        // migrate contract
        let res = migrate(deps.as_mut(), env, MigrateMsg {}).unwrap();

        assert_eq!(res, Response::new());
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => get_config(deps),
        QueryMsg::Vault { asset_info } => get_vault(deps, asset_info),
        QueryMsg::Vaults { start_after, limit } => get_vaults(deps, start_after, limit),
    }
}
