use std::net::SocketAddr;
use std::sync::Arc;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::connection_cache;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::nonblocking::tpu_client::TpuClient;
use solana_client::thin_client::ThinClient;
use solana_client::tpu_client::TpuClientConfig;

use solana_program::instruction::Instruction;
use solana_program::message::Message;
use solana_program::pubkey::Pubkey;
use solana_sdk::client::AsyncClient;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

#[derive(BorshSerialize, BorshDeserialize)]
enum BankInstruction {
    Initialize,
    Deposit { lamports: u64 },
    Withdraw { lamports: u64 },
}

async fn tpuclientsendtransaction(program_id: Pubkey, payer: &Keypair) {
    let c = RpcClient::new("http://0.0.0.0:8899/".to_string());
    let wurl = "ws://localhost:8900";
    let client = TpuClient::new(Arc::new(c), wurl, TpuClientConfig::default())
        .await
        .unwrap();

    let bankins = BankInstruction::Initialize;
    let instruction = Instruction::new_with_borsh(program_id, &bankins, vec![]);

    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let blockhash = client.rpc_client().get_latest_blockhash().await.unwrap();
    let tx = Transaction::new(&[payer], message, blockhash);

    let x = client.send_transaction(&tx).await;
    println!("{}", x);
}

fn thinclientsendtransaction(program_id: Pubkey, payer: &Keypair) {
    let rpc_addr = "127.0.0.1:8899".parse::<SocketAddr>().unwrap();
    let tpu_addr = "127.0.0.1:1027".parse::<SocketAddr>().unwrap();
    let connectioncache = connection_cache::ConnectionCache::new(20000000);
    let client = ThinClient::new(rpc_addr, tpu_addr, Arc::new(connectioncache));
    let bankins = BankInstruction::Initialize;
    let instruction = Instruction::new_with_borsh(program_id, &bankins, vec![]);

    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let blockhash = client.rpc_client().get_latest_blockhash().unwrap();
    let tx = Transaction::new(&[payer], message, blockhash);
    let x = client.async_send_transaction(tx).unwrap();
    //let x=client.get_account(&program_id).unwrap();
    println!("{:#?}", x);
}

fn main() {
    let program_id = Pubkey::new_unique();
    let payer = Keypair::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(tpuclientsendtransaction(program_id, &payer));
    thinclientsendtransaction(program_id, &payer);
}
