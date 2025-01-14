use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use terraswap::asset::AssetInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Adds a factory to the fee collector so it can be queried when collecting fees
    AddFactory { factory_addr: String },
    /// Removes a factory from the fee collector
    RemoveFactory { factory_addr: String },
    /// Collects protocol fees based on the configuration indicated by [CollectFeesFor]
    CollectFees { collect_fees_for: CollectFeesFor },
    /// Updates the config
    UpdateConfig { owner: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollectFeesFor {
    /// Collects the fees accumulated by the given contracts
    Contracts { contracts: Vec<String> },
    /// Collects the fees accumulated by the contracts the given factory created
    Factory {
        factory_addr: String,
        factory_type: FactoryType,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Queries factories added to the fee collector
    Factories { limit: Option<u32> },
    /// Queries the configuration of this contract
    Config {},
    /// Queries fees collected by a given factory's children or individual contracts
    Fees {
        query_fees_for: QueryFeesFor,
        all_time: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FactoriesResponse {
    pub factories: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FactoryType {
    /// Vault Factory
    Vault {
        start_after: Option<Vec<u8>>,
        limit: Option<u32>,
    },
    /// Pool Factory
    Pool {
        start_after: Option<[AssetInfo; 2]>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryFeesFor {
    /// Specifies list of [Contract]s to query fees for
    Contracts { contracts: Vec<Contract> },
    /// Defines a factory for which to query fees from its children
    Factory {
        factory_addr: String,
        factory_type: FactoryType,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Contract {
    pub address: String,
    pub contract_type: ContractType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContractType {
    /// Vault contract type
    Vault {},
    /// Pool/Pair contract type
    Pool {},
}
