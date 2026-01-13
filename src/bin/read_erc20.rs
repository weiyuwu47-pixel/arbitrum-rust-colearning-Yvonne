use ethers::prelude::*;
use ethers::abi::Abi;       // ✅ 加这一行
use std::sync::Arc;
use std::error::Error;
use serde_json;              // ✅ 加这一行


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 连接 Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    // 2. 合约地址
    let contract_address: Address =
        "0x980b62da83eff3d4576c647993b0c1d7faf17c73".parse()?;


    // 3. ERC20 最小 ABI（只读）
    let abi: Abi = serde_json::from_str(r#"
    [
        {
            "constant": true,
            "inputs": [],
            "name": "name",
            "outputs": [{ "name": "", "type": "string" }],
            "type": "function"
        },
        {
            "constant": true,
            "inputs": [],
            "name": "symbol",
            "outputs": [{ "name": "", "type": "string" }],
            "type": "function"
        },
        {
            "constant": true,
            "inputs": [],
            "name": "decimals",
            "outputs": [{ "name": "", "type": "uint8" }],
            "type": "function"
        }
    ]
    "#)?;
        // 4. 加载合约
    let contract = Contract::new(contract_address, abi, client);

    // 5. 调用只读方法
    let name: String = contract
        .method::<_, String>("name", ())?
        .call()
        .await?;

    let symbol: String = contract
        .method::<_, String>("symbol", ())?
        .call()
        .await?;

    let decimals: u8 = contract
        .method::<_, u8>("decimals", ())?
        .call()
        .await?;

    println!("Token Name: {}", name);
    println!("Symbol: {}", symbol);
    println!("Decimals: {}", decimals);

    Ok(())
}

