use engine::{
    types::{Account, Transaction},
    Engine,
};
use std::error::Error;
use std::io::{Read, Write};

mod engine;

pub fn process_transactions(
    reader: impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new();

    let mut csv_reader = csv::Reader::from_reader(reader);
    for record in csv_reader.records() {
        let record = record?;
        let fields: Vec<&str> = record.iter().map(str::trim).collect();
        let transaction = parse_transaction(&fields)?;
        engine.do_transaction(transaction);
    }

    let mut csv_writer = csv::Writer::from_writer(writer);
    let account = Account::new(1);
    csv_writer.write_record([
        account.client.to_string(),
        account.available.to_string(),
        account.held.to_string(),
        (account.available + account.held).to_string(),
        account.locked.to_string(),
    ])?;

    Ok(())
}

fn parse_transaction(fields: &[&str]) -> Result<Transaction, Box<dyn Error>> {
    return if let [tx_type, client, tx, amount] = fields {
        Ok(match *tx_type {
            "deposit" => Transaction::Deposit {
                id: tx.parse()?,
                client: client.parse()?,
                amount: amount.parse()?,
            },
            "withdrawal" => Transaction::Withdrawal {
                id: tx.parse()?,
                client: client.parse()?,
                amount: amount.parse()?,
            },
            "dispute" => Transaction::Dispute {
                client: client.parse()?,
                deposit: tx.parse()?,
            },
            "resolve" => Transaction::Resolve {
                client: client.parse()?,
                deposit: tx.parse()?,
            },
            "chargeback" => Transaction::Chargeback {
                client: client.parse()?,
                deposit: tx.parse()?,
            },
            _ => return Err(format!("invalid transaction type: {}", tx_type).into()),
        })
    } else {
        Err("invalid number of fields in CSV line".into())
    };
}
