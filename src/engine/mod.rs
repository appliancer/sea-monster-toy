use std::collections::HashMap;
use types::*;

pub mod types;

pub struct Engine {
    clients: HashMap<ClientId, Account>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            clients: HashMap::new(),
        }
    }

    pub fn do_transaction(&mut self, transaction: Transaction) {
        eprintln!("doing transaction: {:?}", transaction);
    }
}
