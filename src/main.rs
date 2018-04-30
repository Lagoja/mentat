// extern crate pencil;
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate crypto;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use lib::blockchain::{Block, Blockchain, Transaction, NodeList};
use lib::response_types::{FullChainResponse, MineResponse, NodeRegResponse, NodeResolveResponse};
use rocket::response::content;
use rocket::State;
use rocket_contrib::Json;
use serde::Serialize;
use std::sync::RwLock;
use uuid::Uuid;

mod lib;

pub struct BlockchainState {
    pub blockchain: RwLock<Blockchain>,
}

type JsonResult = Result<content::Json<String>, String>;

impl BlockchainState {
    fn new() -> BlockchainState {
        BlockchainState {
            blockchain: RwLock::new(Blockchain::new())
        }
    }
}

#[get("/mine")]
pub fn mine(bc: State<BlockchainState>, node_identifier: State<Uuid>) -> JsonResult {
    match bc.blockchain.write() {
        Ok(mut blockchain) => {
            match blockchain.mine(&node_identifier.inner()) {
                Ok(block) => to_json(mine_response(block)),
                Err(e) => Err(e.to_string())
            }
        }
        Err(e) => Err(e.to_string())
    }
}

#[post("/transactions/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(bc: State<BlockchainState>, transaction: Json<Transaction>) -> JsonResult {
    match bc.blockchain.write() {
        Ok(mut blockchain) => {
            blockchain.new_transaction(&transaction.sender, &transaction.recipient, transaction.amount);
            to_json("'message' : 'New transaction added to block")
        }
        Err(e) => Err(e.to_string())
    }
}

#[get("/transactions")]
pub fn transactions(bc: State<BlockchainState>) -> JsonResult {
    match bc.blockchain.read() {
        Ok(blockchain) => to_json(&blockchain.transactions()),
        Err(e) => Err(e.to_string())
    }
}

#[get("/chain")]
pub fn full_chain(bc: State<BlockchainState>) -> JsonResult {
    match bc.blockchain.read() {
        Ok(blockchain) => to_json(chain(&blockchain)),
        Err(e) => Err(e.to_string())
    }
}

#[get("/nodes")]
pub fn nodes(bc: State<BlockchainState>) -> JsonResult{
    match bc.blockchain.read(){
        Ok(blockchain) => to_json(&blockchain.nodes),
        Err(e) => Err(e.to_string())
    }
}

#[post("/nodes/register", format = "application/json", data = "<node_list>")]
pub fn register_nodes(bc: State<BlockchainState>, node_list: Json<NodeList>) -> JsonResult {
    match bc.blockchain.write(){
        Ok(mut blockchain) => {
            for node in node_list.node_list.clone() {
                blockchain.register_node(node);
            }
            to_json(node_reg(&blockchain))
        },
        Err(e) => Err(e.to_string())
    }
}

#[get("/nodes/resolve",)]
pub fn resolve_nodes(bc: State<BlockchainState>) -> JsonResult {
    match bc.blockchain.write(){
        Ok(mut blockchain) => {
            if blockchain.resolve_conflicts() {
                to_json(node_resolve(String::from("Our chain was replaced"), &blockchain))
            } else {
                to_json(node_resolve(String::from("Our chain is authoritative"), &blockchain))
            }
        },
        Err(e) => Err(e.to_string())
    }
}

//API + Utility Functions

pub fn chain(b: &Blockchain) -> FullChainResponse {
    let chain = b.chain();
    FullChainResponse {
        chain,
        length: chain.len() as u64,
    }
}

pub fn node_reg(b: &Blockchain) -> NodeRegResponse{
    let nodes = &b.nodes;
    NodeRegResponse{
        message: String::from("New Nodes have been added"),
        nodes: nodes,
    }
}

pub fn node_resolve(message: String, b: &Blockchain) -> NodeResolveResponse{
    let chain = b.chain();
    NodeResolveResponse{
        message,
        full_chain_response: FullChainResponse{
          chain: chain,
          length: chain.len() as u64 
        },
    }
}

pub fn mine_response(b: &Block) -> MineResponse {
    MineResponse {
        message: String::from("Returning Block"),
        index: b.index,
        transactions: b.transactions.clone(),
        previous_hash: b.previous_hash.clone(),
        proof: b.proof,
    }
}

pub fn to_json<T>(response: T) -> JsonResult
    where T: Serialize {
    match serde_json::to_string(&response) {
        Ok(serialized) => Ok(content::Json(serialized)),
        Err(err) => Err(err.to_string())
    }
}

fn main() {
    let b: BlockchainState = BlockchainState::new();
    let id: Uuid = Uuid::new_v4();
    rocket::ignite()
        .manage(b)
        .manage(id)
        .mount("/", routes![mine, new_transaction, transactions, full_chain, nodes, register_nodes, resolve_nodes])
        .launch();
}
