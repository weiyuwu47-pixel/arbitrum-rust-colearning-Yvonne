use ethers::prelude::*;
use ethers::utils; // 👈 关键 1：引入 utils
use ethers::signers::coins_bip39::English; // 👈 关键 2：引入 English
use std::env;
use std::sync::Arc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 连接 Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 2. 从环境变量读取助记词
    let mnemonic = env::var("ARB_MNEMONIC")
        .expect("请先设置 ARB_MNEMONIC 环境变量");

    // 3. 用助记词派生第 0 个账户（MetaMask 默认账户）
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .build()?;

    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = wallet.with_chain_id(chain_id);

    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // 4. 转账参数
    let from = client.address();
    let to: Address = "0x51DA7FCA8dc95520658838c594465e84193d9b33".parse()?; // ⚠️ 注意：你刚才多写了一个 0x
    let value = utils::parse_ether("0.001")?;

    println!("From: {:?}", from);
    println!("To:   {:?}", to);

    // 5. 构造交易
    let gas_price = client.get_gas_price().await?;
    println!("当前 Gas Price: {}", gas_price);

    let tx = TransactionRequest::new()
        .to(to)
        .value(value)
        .gas_price(gas_price * 2); // 👈 关键：给它翻倍，防止 baseFee 波动


    // 6. 发送交易（帮 Rust 明确类型）
    let pending_tx: PendingTransaction<'_, Http> =
        client.send_transaction(tx, None).await?;

    let tx_hash = pending_tx.tx_hash();

    println!("交易已发送！");
    println!("Tx Hash: {:?}", tx_hash);

    Ok(())
}
