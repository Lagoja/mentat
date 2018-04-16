use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time;
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
use rocket::{Request, Data, Outcome};
use rocket::Outcome::*;
use serde_json;
use uuid::Uuid;

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

#[derive(Clone, Serialize, PartialEq, Debug)]
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
    pub current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut b = Blockchain {
            chain: vec![],
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

    pub fn chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn transactions(&self) -> &Vec<Transaction> {&self.current_transactions}

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
        while !self.valid_proof(last_proof, proof) {
            proof += 1;
        }

        proof
    }

    pub fn last_block_hash(&self) -> String {
        let last_block = self.last_block().unwrap();
        Blockchain::hash_block(last_block)
    }

    pub fn mine(&mut self, node_identifier: &Uuid) -> Result<&Block, String>{
        let proof = self.proof_of_work();
        self.new_transaction("0", &(node_identifier).simple().to_string(), 1);
        let previous_hash = self.last_block_hash();
        let new_block = self.new_block(previous_hash, proof);
        Ok(&new_block)
    }

    pub fn valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        let mut hasher = Sha256::new();
        let mut guess = last_proof.to_string();
        let p_str = proof.to_string();

        guess.push_str(&p_str);
        hasher.input_str(&guess[..]);

        hasher.result_str()[..4].eq("0000")
    }
}
