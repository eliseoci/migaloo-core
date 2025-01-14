use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use fee_collector::msg::{
    CollectFeesFor, Contract, ContractType, ExecuteMsg, FactoriesResponse, FactoryType,
    InstantiateMsg, MigrateMsg, QueryFeesFor, QueryMsg,
};
use fee_collector::state::{Config, ConfigResponse};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(FactoriesResponse), &out_dir);
    export_schema(&schema_for!(FactoryType), &out_dir);
    export_schema(&schema_for!(CollectFeesFor), &out_dir);
    export_schema(&schema_for!(QueryFeesFor), &out_dir);
    export_schema(&schema_for!(Contract), &out_dir);
    export_schema(&schema_for!(ContractType), &out_dir);
    export_schema_with_title(&schema_for!(ConfigResponse), &out_dir, "ConfigResponse");
}
