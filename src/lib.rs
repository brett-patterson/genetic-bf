#[macro_use]
extern crate serde_derive;

mod config;
mod gen;
mod vm;

pub use config::Config;
pub use gen::generate_program;
pub use vm::VM;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
