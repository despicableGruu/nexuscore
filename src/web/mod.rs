use serde_json;
use std::collections::BTreeSet;
use rocket::State;
use lib::blockchain::*;
use lib::transaction::*;
use std::sync::{RwLock};

mod converters;

pub struct BlockchainState {
    pub blockchain: RwLock<Blockchain>
}

impl BlockchainState {
    pub fn new() -> BlockchainState {
        BlockchainState {
            blockchain: RwLock::new(Blockchain::new())
        }
    }
}

#[derive(Serialize)]
struct MineResult {
    message: String,
    index: usize,
    transactions: BTreeSet<Transaction>,
    proof: u64,
    previous_hash: String
}

//todo: respone as JSON - https://github.com/SergioBenitez/Rocket/blob/v0.3.3/examples/json/src/main.rs
#[get("/mine", format = "application/json")]
pub fn mine(state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b| {

        let mined_block = b.mine();

        let response = MineResult {
            message: "New Block Forged".into(),
            index: mined_block.index,
            transactions: mined_block.transactions.clone(),
            proof: mined_block.proof,
            previous_hash: mined_block.previous_hash.clone()
        };

        serde_json::to_string(&response).unwrap_or_else(|e| {
            error!("serialize errro: {:?}", e);
            return String::from("Block mined, but details not available")
        })
    });
}

//todo: post
#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| {
        let index = b.new_transaction(transaction.clone());
        return format!("Transaction added at block {}", index);
    })
}



///
/// Retrieves the blockchain from state, unlocks and executions the closure
/// 
fn blockchain_op<F>(state: &State<BlockchainState>, blockchain_op: F) -> Result<String, u32> 
    where F: Fn(&mut Blockchain) -> String {
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let result = blockchain_op(&mut blockchain);
        return Ok(result);        
    }
    Err(500)
}


