mod vm;

use std::io::{Read, Write};
use vm::VM;

pub fn run_program<I, O>(prog: Vec<u8>, input: fn() -> I, output: fn() -> O) where I: Read, O: Write {
    let mut vm = VM::new(prog, input, output);
    vm.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
