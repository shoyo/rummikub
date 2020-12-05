/// Copyright (c) 2020, Shoyo Inokuchi
use rummikub::parser::is_valid_set;
use rummikub::tiles::deserialize_set;
use std::io::{self, Write};

fn main() {
    println!("Input a tile sequence:");
    let mut set = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read from stdin");

        match deserialize_set(buf.trim()) {
            Ok(s) => set = s,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        match is_valid_set(&set) {
            true => println!("Valid set."),
            false => println!("Invalid set."),
        }
    }
}
