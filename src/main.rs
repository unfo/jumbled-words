use jumbled_words;
use std::io::{self, Write};
use std::env;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <number_of_megabytes>", args[0]);
        std::process::exit(1);
    }
    let seed = 1u64;
    let megabytes: usize = args[1].parse().expect("Please enter a valid number");
    let data = jumbled_words::generate_random_data(megabytes, seed);

    io::stdout().write_all(&data)?;

    Ok(())
}


