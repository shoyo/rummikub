use rummikub::parser::is_valid_set;
use rummikub::tiles::deserialize_set;
/// Copyright (c) 2020, Shoyo Inokuchi
use std::io::{self, Write};

fn main() {
    println!("Input a tile sequence:");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read from stdin");

        let set = match deserialize_set(buf) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        };

        match is_valid_set(&set) {
            true => println!("Valid set."),
            false => println!("Invalid set."),
        }
    }
}
