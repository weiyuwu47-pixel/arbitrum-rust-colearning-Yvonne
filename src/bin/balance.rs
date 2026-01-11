use ethers::prelude::*;
use std::sync::Arc;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Arbitrum Sepolia RPC
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 2. 要查询的地址（换成自己的）
    let address = Address::from_str("0x126Ed668cDA8C8d64622311cf9c49EB38786cD96")?;

    // 3. 查询余额（返回 wei）
    let balance_wei = provider.get_balance(address, None).await?;

    // 4. wei -> ETH
    let balance_eth = ethers::utils::format_ether(balance_wei);

    // 5. 打印结果
    println!("Address: {:?}", address);
    println!("Balance: {} ETH", balance_eth);

    Ok(())
}
