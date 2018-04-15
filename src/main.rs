// extern crate pencil;
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate mentat;
extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;
extern crate serde;
extern crate serde_json;

use mentat::{Blockchain, FullChain, Transaction};
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::Json;
use rocket::response::content;
use std::rc::Rc;
use std::sync::RwLock;
use serde::Serialize;
use uuid::Uuid;

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
pub fn mine() -> &'static str {
    "We'll mine a new block here"
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
pub fn transactions(bc: State<BlockchainState>) -> JsonResult{
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

pub fn chain(b: &Blockchain) -> FullChain {
    let chain = b.chain();
    FullChain {
        chain,
        length: chain.len() as u64,
    }
}

pub fn to_json<T>(response: T) -> JsonResult
    where T: Serialize {
    match serde_json::to_string(&response){
        Ok(serialized) => Ok(content::Json(serialized)),
        Err(err) => Err(err.to_string())
    }
}

#[get("/hello/<name>")]
pub fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}


fn main() {
    let b: BlockchainState = BlockchainState::new();
    let id: Uuid = Uuid::new_v4();
    rocket::ignite()
        .manage(b)
        .manage(id)
        .mount("/", routes![mine, new_transaction, transactions, full_chain, hello])
        .launch();
}
