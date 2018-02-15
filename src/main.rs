extern crate parsing_experiments;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use parsing_experiments::parse_ascii_stl;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    if args.len() < 2 {
        println!("No file path given on command line");
    } else {
        let filename = &args[1];
        println!("Trying to parse file at {}", filename);

        let mut file = File::open(filename).expect("failed to open file");
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer).unwrap();
        
        let faces = parse_ascii_stl(&buffer).unwrap().1;
        println!("{:?}", faces);
    }
}
