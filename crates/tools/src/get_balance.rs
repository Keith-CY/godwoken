use anyhow::Result;

use crate::{account::parse_account_from_str, godwoken_rpc::GodwokenRpcClient};

pub fn get_balance(godwoken_rpc_url: &str, account: &str, sudt_id: u32) -> Result<()> {
    let mut godwoken_rpc_client = GodwokenRpcClient::new(godwoken_rpc_url);
    let script_hash = parse_account_from_str(&mut godwoken_rpc_client, account)?;
    let addr = godwoken_rpc_client.get_registry_address_by_script_hash(&script_hash)?;
    let balance = godwoken_rpc_client.get_balance(&addr, sudt_id)?;
    log::info!("Balance: {}", balance);

    Ok(())
}
