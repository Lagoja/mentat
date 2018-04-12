
extern crate mentat;

use mentat::Blockchain;

#[test]
fn create_blockchain() {
    let chain = vec![];
    let current_transactions = vec![];

    assert_eq!(Blockchain::new(), Blockchain{
        chain,
        current_transactions,
    });
}

#[test]
fn valid_proof_check_fail() {
    let last_proof = 23;
    let proof = 42;

    assert_eq!(Blockchain::new().valid_proof(last_proof, proof), false);
}

#[test]
fn valid_proof_pass() {
    let last_proof = 88;
    let proof = 484; // 88484 generates a SHA256 with 4 leading 0's

    assert_eq!(Blockchain::new().valid_proof(last_proof, proof), true);
}

