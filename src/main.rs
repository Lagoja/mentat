// extern crate pencil;
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate mentat;
extern crate rocket;
extern crate uuid;
extern crate rocket_contrib;

use mentat::{Blockchain, FullChain};
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::Json;
use uuid::Uuid;

#[get("/mine")]
fn mine() -> &'static str {
    "We'll mine a new block here"
}

#[post("/transactions/new")]
fn new_transaction() -> &'static str {
    "We'll add a new transaction"
}

#[get("/chain")]
fn full_chain(bc: State<Blockchain>) -> Json<FullChain> {
    let c = bc.chain.clone();
    Json(FullChain{
        chain: c,
        length: bc.chain.len() as u64,
    })
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}


fn main() {
    let blockchain: Blockchain = Blockchain::new();
    let id: Uuid = Uuid::new_v4();
    rocket::ignite()
        .manage(blockchain)
        .manage(id)
        .mount("/", routes![mine, new_transaction, full_chain, hello])
        .launch();
}
