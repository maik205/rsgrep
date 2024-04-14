use std::{env, fs, ops::Deref, path::Display};

use rsgrep::parse_arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("rsgrep: There must be more than 2 arguments.");
        return ();
    }
    println!("{:?}", args);

    println!("{:?}", parse_arguments(args));
}

