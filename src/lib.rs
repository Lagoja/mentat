extern crate crypto;
extern crate serde;
extern crate serde_json;
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time;
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
use rocket::{Request, Data, Outcome};
use rocket::Outcome::*;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u32,
}

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct FullChain<'a>{
    pub chain: &'a Vec<Block>,
    pub length: u64,
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
    index: u32,
    timestamp: time::SystemTime,
    transactions: Vec<Transaction>,
    proof: u32,
    previous_hash: u32,
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
        self.new_block(1, 100);
    }

    fn new_block(&mut self, previous_hash: u32, proof: u32) {
        let block = Block {
            index: (self.chain.len() + 1) as u32,
            timestamp: time::SystemTime::now(),
            transactions: self.current_transactions.clone(),
            proof,
            previous_hash,
        };

        self.clear_transactions();
        self.chain.push(block);
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

    fn hash(block: Block) -> String {
        let block_string = serde_json::to_string(&block).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(&block_string[..]);
        hasher.result_str()
    }

    fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    fn proof_of_work(&self, last_proof: u32) -> u32 {
        let mut proof: u32 = 0;
        while !self.valid_proof(last_proof, proof) {
            proof += 1;
        }

        proof
    }

    pub fn valid_proof(&self, last_proof: u32, proof: u32) -> bool {
        let mut hasher = Sha256::new();
        let mut guess = last_proof.to_string();
        let p_str = proof.to_string();

        guess.push_str(&p_str);
        hasher.input_str(&guess[..]);

        hasher.result_str()[..4].eq("0000")
    }
}
