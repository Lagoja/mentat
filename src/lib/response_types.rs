use lib::blockchain::{Block, Transaction};
use std::collections::hash_set::HashSet;

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct FullChainResponse<'a>{
    pub chain: &'a Vec<Block>,
    pub length: u64,
}


#[derive(Serialize)]
pub struct NodeResolveResponse<'a>{
    pub message: String,
    pub full_chain_response: FullChainResponse<'a>
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

#[derive(Serialize)]
pub struct NodeRegResponse<'a> {
    pub message: String,
    pub nodes: &'a HashSet<String>,
}

