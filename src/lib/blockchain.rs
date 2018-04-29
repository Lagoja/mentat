use crypto::digest::Digest;
use crypto::sha2::Sha256;
use futures::{Future, Stream};
use hyper::{Client, Uri};
use hyper::{Chunk, StatusCode};
use lib::response_types::FullChainResponse;
use rocket::data::{self, FromData};
use rocket::http::{ContentType, Status};
use rocket::Outcome::*;
use serde_json;
use std::collections::hash_set::HashSet;
use std::time;
use tokio_core::reactor::Core;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ChainResponse{
    pub chain: Vec<Block>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u32,
}

impl Transaction {
    fn new(sender: &str, recipient: &str, amount: u32) -> Transaction {
        Transaction {
            sender: String::from(sender),
            recipient: String::from(recipient),
            amount,
        }
    }
}

#[derive(Clone, Serialize, PartialEq, Debug, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: time::SystemTime,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

#[derive(PartialEq, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub nodes: HashSet<String>,
    pub current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut b = Blockchain {
            chain: vec![],
            nodes: HashSet::new(),
            current_transactions: vec![],
        };

        b.genesis_block();
        b
    }

    pub fn genesis_block(&mut self) {
        self.new_block("1".to_string(), 100);
    }

    pub fn new_block(&mut self, previous_hash: String, proof: u64) -> &Block {
        let block = Block {
            index: (self.chain.len() + 1) as u32,
            timestamp: time::SystemTime::now(),
            transactions: self.current_transactions.clone(),
            proof,
            previous_hash,
        };

        self.clear_transactions();
        self.chain.push(block);
        self.chain.iter().next_back().expect("Failed to get last block")
    }

    pub fn register_node(&mut self, address: String) {
        self.nodes.insert(address);
    }

    pub fn valid_chain(chain: &Vec<Block>) -> bool {
        /* Determine if a blockchain is valid */
        let mut last_block = &chain[0]; //mutable borrow
        let mut current_index = 1;  //move

        while current_index < chain.len() {
            let block = &chain[current_index]; // immutable borrow
            if block.previous_hash != Blockchain::hash_block(last_block) {
                return false;
            };

            if !Blockchain::valid_proof(last_block.proof, block.proof) {
                return false;
            };

            last_block = block;
            current_index += 1;
        };

        true
    }

    pub fn new_consensus (max_length: usize, new_chain: &Vec<Block>) -> bool{
       if new_chain.len() > max_length && Blockchain::valid_chain(new_chain){
           true
       } else {
           false
       }
    }

    pub fn resolve_conflicts(&mut self) -> bool {
        let neighbors = &self.nodes;
        let mut new_chain: Vec<Block> = vec![];
        let mut max_length = self.chain.len();

        let core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        for node in neighbors.iter() {
            let url: Uri = node.parse().unwrap();

            let request = client.get(url)
                .and_then(|res| {
                    let status = res.status();
                    if status == StatusCode::Ok {
                        res.body().concat2().and_then(|body: Chunk| {
                            let cr: ChainResponse = serde_json::from_slice::<ChainResponse>(&body).unwrap();
                            if Blockchain::new_consensus(max_length, &cr.chain){
                                max_length = cr.chain.len();
                                new_chain = cr.chain;
                            };
                            Ok(())
                        });
                    }
                    Ok(())
                });
        }

        false
    }

    pub fn chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn transactions(&self) -> &Vec<Transaction> { &self.current_transactions }

    fn clear_transactions(&mut self) {
        self.current_transactions = vec![]
    }

    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: u32) -> u32 {
        self.current_transactions
            .push(Transaction::new(sender, recipient, amount));
        self.last_block().unwrap().index - 1
    }

    pub fn hash_block(block: &Block) -> String {
        let block_string = serde_json::to_string(&block).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(&block_string[..]);
        hasher.result_str()
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn proof_of_work(&self) -> u64 {
        let last_block = self.last_block().expect("Error retrieving last block");
        let last_proof = last_block.proof;
        let mut proof: u64 = 0;
        while !Blockchain::valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    pub fn last_block_hash(&self) -> String {
        let last_block = self.last_block().unwrap();
        Blockchain::hash_block(last_block)
    }

    pub fn mine(&mut self, node_identifier: &Uuid) -> Result<&Block, String> {
        let proof = self.proof_of_work();
        self.new_transaction("0", &(node_identifier).simple().to_string(), 1);
        let previous_hash = self.last_block_hash();
        let new_block = self.new_block(previous_hash, proof);
        Ok(&new_block)
    }

    pub fn valid_proof(last_proof: u64, proof: u64) -> bool {
        let mut hasher = Sha256::new();
        let mut guess = last_proof.to_string();
        let p_str = proof.to_string();

        guess.push_str(&p_str);
        hasher.input_str(&guess[..]);

        hasher.result_str()[..4].eq("0000")
    }
}
