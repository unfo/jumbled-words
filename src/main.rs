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
    let megabytes: usize = args[1].parse().expect("Please enter a valid number");

    // Create a data structure with the specified number of bytes
    let mut data = vec![0u8; megabytes * 1024 * 1024];

    // Apply a function to manipulate the data (identity function in this case)
    data = jumbled_words::manipulate_data(data);

    // Write the data to standard out
    io::stdout().write_all(&data)?;

    Ok(())
}
