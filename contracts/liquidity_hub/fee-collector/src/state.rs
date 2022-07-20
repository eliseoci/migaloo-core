use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_storage_plus::{Bound, Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

pub type ConfigResponse = Config;

pub const CONFIG: Item<Config> = Item::new("config");
pub const FACTORIES: Map<&[u8], Addr> = Map::new("factories");

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 2;

pub fn read_factories(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<Addr>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    if let Some(factory) = start_after {
        FACTORIES
            .range(
                deps.storage,
                Some(Bound::exclusive(
                    deps.api.addr_validate(factory.as_str())?.as_bytes(),
                )),
                None,
                Order::Ascending,
            )
            .take(limit)
            .map(|item| {
                let (_, factory_addr) = item?;
                Ok(factory_addr)
            })
            .collect()
    } else {
        FACTORIES
            .range(deps.storage, None, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                let (_, factory_addr) = item?;
                Ok(factory_addr)
            })
            .collect()
    }
}