use std::net::SocketAddr;
use std::sync::Arc;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_client::connection_cache;
use solana_client::nonblocking::tpu_client::TpuClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::thin_client::ThinClient;
use solana_client::tpu_client::TpuClientConfig;

use solana_program::instruction::Instruction;
use solana_program::message::Message;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;


#[derive(BorshSerialize, BorshDeserialize)]
enum BankInstruction {
    Initialize,
    Deposit { lamports: u64 },
    Withdraw { lamports: u64 },
}

async fn tpuclientsendtransaction(program_id: Pubkey, payer: &Keypair){
    let c=RpcClient::new("http://0.0.0.0:8899/".to_string());
    let wurl="ws://localhost:8900";
    let client=TpuClient::new(Arc::new(c), wurl, TpuClientConfig::default()).await.unwrap();
    
    let bankins=BankInstruction::Initialize;
    let instruction = Instruction::new_with_borsh(
        program_id,
        &bankins,
        vec![],
    );

    let message = Message::new(
        &[instruction],
        Some(&payer.pubkey()),
    );
    let blockhash = client.rpc_client().get_latest_blockhash().await.unwrap();
    let tx = Transaction::new(&[payer], message, blockhash);
    
    let x=client.send_transaction(&tx).await;
    println!("{}",x);
}

fn lightclientsendtransaction(program_id: Pubkey, payer: &Keypair){
    let rpc_addr="0.0.0.0:8899".parse::<SocketAddr>().unwrap();
    let tpu_addr="0.0.0.0:1027".parse::<SocketAddr>().unwrap();
    let connectioncache=connection_cache::ConnectionCache::new(2000);
    let client=ThinClient::new(rpc_addr, tpu_addr, Arc::new(connectioncache));
    let bankins=BankInstruction::Initialize;
    let instruction = Instruction::new_with_borsh(
        program_id,
        &bankins,
        vec![],
    );
    
    let message = Message::new(
        &[instruction],
        Some(&payer.pubkey()),
    );

    let blockhash = client.rpc_client().get_latest_blockhash().unwrap();
    let mut tx = Transaction::new(&[payer], message, blockhash);
    let x=client.send_and_confirm_transaction(&[payer], &mut tx, 20000, 0).unwrap();
    println!("{x}");
}

fn main() {
    let program_id=Pubkey::new_unique();
    let payer=Keypair::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(tpuclientsendtransaction(program_id, &payer));
    lightclientsendtransaction(program_id, &payer);
}
