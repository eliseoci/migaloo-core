use cw_storage_plus::Item;
use terraswap::asset::Asset;
use vault_network::vault::Config;

pub const CONFIG: Item<Config> = Item::new("config");

pub const COLLECTED_PROTOCOL_FEES: Item<Asset> = Item::new("collected_protocol_fees");
pub const ALL_TIME_COLLECTED_PROTOCOL_FEES: Item<Asset> =
    Item::new("all_time_collected_protocol_fees");

// A counter for how many active loans are being performed
pub const LOAN_COUNTER: Item<u32> = Item::new("loan_counter");
