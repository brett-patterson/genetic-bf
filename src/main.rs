extern crate genetic_bf;
extern crate clap;
extern crate serde_yaml;

use std::fs::File;
use std::io::{Read, Error, ErrorKind, stdin, stdout};

use clap::{Arg, App};

use genetic_bf::{Config, VM, generate_program};

fn main() {
    let args = App::new("genetic-bf")
        .version("0.1.0")
        .author("Brett Patterson")
        .about("An attempt at using genetic algorithms to discover Brainfuck programs")
        .arg(Arg::with_name("run")
            .short("r")
            .long("run")
            .help("Run the provided Brainfuck program"))
        .arg(Arg::with_name("file")
            .help("The input configuration or Brainfuck program")
            .required(true))
        .get_matches();

    let file = args.value_of("file")
        .ok_or(Error::new(ErrorKind::InvalidInput, "No file specified"))
        .and_then(File::open);

    match file {
        Ok(file) => {
            if args.is_present("run") {
                let prog: Vec<u8> = file.bytes().collect::<Result<Vec<u8>, Error>>().unwrap();
                let mut vm = VM::new(prog, stdin, stdout);
                vm.run();
            } else {
                let config: Config = serde_yaml::from_reader(file).unwrap();
                match generate_program(config) {
                    Ok(prog) => {
                        println!("{}", prog);
                    }
                    Err(e) => {
                        println!("Unable to generate program: {}", e);
                    }
                }
            }
        }
        Err(e) => panic!("Unable to open file: {}", e),
    }
}