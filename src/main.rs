use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::{env, io, process};

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Missing filename argument");
        process::exit(1);
    });

    let file = File::open(&filename).unwrap_or_else(|err| {
        eprintln!("Error opening file {}: {}", &filename, err);
        process::exit(1);
    });

    process_transactions(file, &mut io::stdout()).unwrap_or_else(|err| {
        eprintln!("Error processing transactions: {}", err);
        process::exit(1);
    });
}

fn process_transactions(reader: impl Read, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    for record in csv_reader.records() {
        let record = record?;
        let fields: Vec<&str> = record.iter().map(str::trim).collect();
        let transaction = parse_transaction(&fields)?;
        writeln!(writer, "{:?}", transaction)?;
    }
    Ok(())
}

#[derive(Debug)]
enum Transaction {
    Deposit { id: u32, client: u16, amount: f64 },
    Withdrawal { id: u32, client: u16, amount: f64 },
    Dispute { client: u16, deposit: u32 },
    Resolve { client: u16, deposit: u32 },
    Chargeback { client: u16, deposit: u32 },
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
