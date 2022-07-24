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
        // TODO: check if account locked
        match transaction {
            Transaction::Deposit { id, client, amount } => self.do_deposit(id, client, amount),
            Transaction::Withdrawal { client, amount, .. } => self.do_withdrawal(client, amount)?,
            Transaction::Dispute { client, deposit } => self.do_dispute(client, deposit)?,
            Transaction::Resolve { client, deposit } => self.do_resolve(client, deposit)?,
            Transaction::Chargeback { client, deposit } => self.do_chargeback(client, deposit)?,
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

    fn do_dispute(&mut self, client: ClientId, deposit: TransactionId) -> Result<(), String> {
        let deposit =
            Engine::get_deposit_mut(&mut self.deposits, deposit, client, DisputeState::Deposited)?;

        let account = self
            .accounts
            .get_mut(&client)
            .ok_or(format!("disputing client {} does not exist", client))?;

        account.available -= deposit.amount;
        account.held += deposit.amount;
        deposit.dispute_state = DisputeState::Disputed;

        Ok(())
    }

    fn do_resolve(&mut self, client: ClientId, deposit: TransactionId) -> Result<(), String> {
        let deposit =
            Engine::get_deposit_mut(&mut self.deposits, deposit, client, DisputeState::Disputed)?;

        let account = self
            .accounts
            .get_mut(&client)
            .ok_or(format!("resolving client {} does not exist", client))?;

        account.held -= deposit.amount;
        account.available += deposit.amount;
        deposit.dispute_state = DisputeState::Resolved;

        Ok(())
    }

    fn do_chargeback(&mut self, client: ClientId, deposit: TransactionId) -> Result<(), String> {
        let deposit =
            Engine::get_deposit_mut(&mut self.deposits, deposit, client, DisputeState::Disputed)?;

        let account = self
            .accounts
            .get_mut(&client)
            .ok_or(format!("chargebacking client {} does not exist", client))?;

        account.held -= deposit.amount;
        account.locked = true;
        deposit.dispute_state = DisputeState::ChargedBack;

        Ok(())
    }

    fn get_deposit_mut(
        deposits: &mut HashMap<TransactionId, Deposit>,
        id: TransactionId,
        client: ClientId,
        expected_dispute_state: DisputeState,
    ) -> Result<&mut Deposit, String> {
        let deposit = deposits
            .get_mut(&id)
            .ok_or(format!("deposit with transaction id {} does not exist", id))?;

        if deposit.client != client {
            return Err(format!(
                "client {} does not match deposit client {}",
                client, deposit.client
            ));
        }

        if deposit.dispute_state != expected_dispute_state {
            return Err(format!(
                "transaction {} has dispute state {:?}, expected {:?}",
                id, deposit.dispute_state, expected_dispute_state
            ));
        }

        Ok(deposit)
    }
}

struct Deposit {
    client: ClientId,
    amount: Money,
    dispute_state: DisputeState,
}

#[derive(Debug, PartialEq)]
enum DisputeState {
    Deposited,
    Disputed,
    Resolved,
    ChargedBack,
}
