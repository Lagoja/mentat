use lib::blockchain::{Block, Transaction};

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct FullChainResponse<'a>{
    pub chain: &'a Vec<Block>,
    pub length: u64,
}

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct NewTransactionResponse {
    pub message: String,
    pub index: u32,
}

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct MineResponse {
    pub message: String,
    pub index: u32,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

