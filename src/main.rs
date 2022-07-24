use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
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

    let reader = BufReader::new(file);

    process_transactions(reader, &mut io::stdout()).unwrap_or_else(|err| {
        eprintln!("Error processing transactions: {}", err);
        process::exit(1);
    });
}

fn process_transactions(
    reader: impl BufRead,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    for line in reader.lines() {
        writeln!(writer, "{}", line?)?;
    }
    Ok(())
}
