use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Arbitrum Sepolia RPC
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 2. 获取实时 Gas Price（单位：wei）
    let gas_price = provider.get_gas_price().await?;
    println!("Current gas price (wei): {}", gas_price);

    // 3. 基础转账 Gas Limit（ETH 普通转账行业通用值）
    let gas_limit: U256 = U256::from(21_000u64);

    // 4. Gas Fee = Gas Price × Gas Limit
    let gas_fee_wei = gas_price * gas_limit;

    // 5. 转换为 ETH（可读格式）
    let gas_fee_eth = ethers::utils::format_ether(gas_fee_wei);

    println!("Estimated transfer gas fee: {} ETH", gas_fee_eth);

    Ok(())
}

