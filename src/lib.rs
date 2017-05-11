#[macro_use]
extern crate serde_derive;
extern crate rand;

mod config;
mod gen;
mod vm;

pub use config::Config;
pub use gen::generate_program;
pub use vm::{VM, VMResult};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
