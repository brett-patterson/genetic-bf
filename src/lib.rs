mod vm;

use vm::VM;

pub fn run_program(prog: Vec<u8>) {
    let mut vm = VM::new(prog);
    vm.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
