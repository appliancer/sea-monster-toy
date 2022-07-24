use std::collections::hash_map::{HashMap, Values};
use types::*;

pub mod types;

pub struct Engine {
    accounts: HashMap<ClientId, Account>,
    deposits: HashMap<TransactionId, Deposit>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            accounts: HashMap::new(),
            deposits: HashMap::new(),
        }
    }

    pub fn get_accounts(&self) -> Values<ClientId, Account> {
        self.accounts.values()
    }

    pub fn do_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        match transaction {
            Transaction::Deposit { id, client, amount } => self.do_deposit(id, client, amount),
            Transaction::Withdrawal { client, amount, .. } => self.do_withdrawal(client, amount)?,
            Transaction::Dispute { client, deposit } => self.do_dispute(client, deposit)?,
            Transaction::Resolve { client, deposit } => {}
            Transaction::Chargeback { client, deposit } => {}
        }
        Ok(())
    }

    fn do_deposit(&mut self, id: TransactionId, client: ClientId, amount: Money) {
        let account = self
            .accounts
            .entry(client)
            .or_insert_with(|| Account::new(client));

        account.available += amount;

        self.deposits.insert(
            id,
            Deposit {
                client,
                amount,
                dispute_state: DisputeState::Deposited,
            },
        ); // TODO: log if already exists
    }

    fn do_withdrawal(&mut self, client: ClientId, amount: Money) -> Result<(), String> {
        let account = self
            .accounts
            .entry(client)
            .or_insert_with(|| Account::new(client));

        if amount > account.available {
            return Err(format!(
                "client {} has insufficient available funds ({}) to withdraw {}",
                account.client, account.available, amount
            ));
        }

        account.available -= amount;

        Ok(())
    }

    fn do_dispute(&mut self, client: ClientId, deposit_id: TransactionId) -> Result<(), String> {
        let deposit = self.deposits.get_mut(&deposit_id).ok_or(format!(
            "deposit with transaction id {} does not exist",
            deposit_id
        ))?;

        if deposit.client != client {
            return Err(format!(
                "dispute client {} does not match deposit client {}",
                client, deposit.client
            ));
        }

        if !matches!(deposit.dispute_state, DisputeState::Deposited) {
            return Err(format!(
                "incorrect dispute state {:?} for transaction {}",
                deposit.dispute_state, deposit_id
            ));
        }

        let account = self
            .accounts
            .get_mut(&client)
            .ok_or(format!("disputing client {} does not exist", client))?;

        account.available -= deposit.amount;
        account.held += deposit.amount;
        deposit.dispute_state = DisputeState::Disputed;

        Ok(())
    }
}

struct Deposit {
    client: ClientId,
    amount: Money,
    dispute_state: DisputeState,
}

#[derive(Debug)]
enum DisputeState {
    Deposited,
    Disputed,
    Resolved,
    ChargedBack,
}
