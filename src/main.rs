extern crate genetic_bf;

use std::env;
use std::fs::File;
use std::io::{Read, Error, stdin, stdout};
use genetic_bf::run_program;

fn main() {
    let mut args = env::args().skip(1);

    if let Some(arg) = args.next() {
        let prog = File::open(arg)
            .and_then(|file| file.bytes().collect::<Result<Vec<u8>, Error>>());

        match prog {
            Ok(prog) => run_program(prog, stdin, stdout),
            Err(e) => panic!("Unable to open file: {}", e),
        }
    }
}