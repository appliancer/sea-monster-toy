pub type ClientId = u16;
pub type TransactionId = u32;
pub type Money = fixed::types::I48F16;

#[derive(Debug)]
pub enum Transaction {
    Deposit {
        id: TransactionId,
        client: ClientId,
        amount: Money,
    },
    Withdrawal {
        id: TransactionId,
        client: ClientId,
        amount: Money,
    },
    Dispute {
        client: ClientId,
        deposit: TransactionId,
    },
    Resolve {
        client: ClientId,
        deposit: TransactionId,
    },
    Chargeback {
        client: ClientId,
        deposit: TransactionId,
    },
}

pub struct Account {
    pub client: ClientId,
    pub available: Money,
    pub held: Money,
    pub locked: bool,
}

impl Account {
    pub fn new(client: ClientId) -> Account {
        Account {
            client,
            available: Default::default(),
            held: Default::default(),
            locked: false,
        }
    }
}
