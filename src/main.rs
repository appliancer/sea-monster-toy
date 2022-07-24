use std::fs::File;
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

    sea_monster_toy::process_transactions(file, &mut io::stdout()).unwrap_or_else(|err| {
        eprintln!("Error processing transactions: {}", err);
        process::exit(1);
    });
}
