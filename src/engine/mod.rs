use std::collections::hash_map::{HashMap, Values};
use types::*;

pub mod types;

pub struct Engine {
    accounts: HashMap<ClientId, Account>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            accounts: HashMap::new(),
        }
    }

    pub fn get_accounts(&self) -> Values<ClientId, Account> {
        self.accounts.values()
    }

    pub fn do_transaction(&mut self, transaction: Transaction) {
        match transaction {
            Transaction::Deposit { id, client, amount } => self.do_deposit(client, amount),
            Transaction::Withdrawal { id, client, amount } => self.do_withdrawal(client, amount),
            Transaction::Dispute { client, deposit } => {}
            Transaction::Resolve { client, deposit } => {}
            Transaction::Chargeback { client, deposit } => {}
        }
    }

    fn do_deposit(&mut self, client: ClientId, amount: Money) {
        let account = self
            .accounts
            .entry(client)
            .or_insert_with(|| Account::new(client));
        account.available += amount;
    }

    fn do_withdrawal(&mut self, client: ClientId, amount: Money) {
        let account = self
            .accounts
            .entry(client)
            .or_insert_with(|| Account::new(client));

        if amount > account.available {
            eprintln!(
                "client {} has insufficient available funds ({}) to withdraw {}",
                account.client, account.available, amount
            );
            return;
        }

        account.available -= amount;
    }
}
