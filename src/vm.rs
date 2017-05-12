use std::u8;
use std::io::{Read, Write};

const DATA_SIZE: usize = 30000;

/// A virtual machine to run a program.
pub struct VM<I: Read, O: Write> {
    prog: Vec<u8>,
    iptr: usize,
    dptr: usize,
    data: [u8; DATA_SIZE],
    input: I,
    output: O,
}

/// An action to be taken by the virtual machine after an instruction
enum VMAction {
    EOF,
    Error(&'static str),
    JumpForward,
    JumpBackward,
    Ok,
}

/// The result of running the machine
pub enum VMResult {
    Error(&'static str),
    Ok
}

impl <I: Read, O: Write> VM<I, O> {
    /// Create a new virtual machine for a program.
    pub fn new(prog: Vec<u8>, input: I, output: O) -> Self {
        VM {
            prog: prog,
            iptr: 0,
            dptr: 0,
            data: [0; DATA_SIZE],
            input: input,
            output: output,
        }
    }

    /// Run the virtual machine.
    pub fn run(&mut self) -> VMResult {
        loop {
            match self.step() {
                VMAction::Ok => {
                    self.iptr += 1;
                }
                VMAction::JumpForward => {
                    self.iptr += 1;

                    let mut count = 1;
                    while count > 0 {
                        match self.prog.get(self.iptr) {
                            Some(byte) => {
                                match byte.clone() as char {
                                    ']' => count -= 1,
                                    '[' => count += 1,
                                    _ => {}
                                }
                            }
                            None => return VMResult::Error("No matching ]"),
                        }

                        self.iptr += 1;
                    }
                }
                VMAction::JumpBackward => {
                    let mut count = 1;
                    while count > 0 {
                        if self.iptr > 0 {
                            self.iptr -= 1;
                        } else {
                            return VMResult::Error("No matching [")
                        }

                        match self.prog.get(self.iptr) {
                            Some(byte) => {
                                match byte.clone() as char {
                                    ']' => count += 1,
                                    '[' => count -= 1,
                                    _ => {}
                                }
                            }
                            None => return VMResult::Error("No matching ["),
                        }
                    }

                    self.iptr += 1;
                }
                VMAction::Error(e) => return VMResult::Error(e),
                VMAction::EOF => break,
            }
        }

        VMResult::Ok
    }

    /// Process the next instruction in the program (as defined by the
    /// instruction pointer), returning the next action to be taken by the VM.
    fn step(&mut self) -> VMAction {
        if self.iptr >= self.prog.len() {
            return VMAction::EOF;
        }

        if let Some(byte) = self.prog.get(self.iptr) {
            match byte.clone() as char {
                '>' => {
                    if self.dptr < DATA_SIZE - 1 {
                        self.dptr += 1;
                        VMAction::Ok
                    } else {
                        VMAction::Error("Data pointer moved out of bounds")
                    }
                }
                '<' => {
                    if self.dptr > 0 {
                        self.dptr -= 1;
                        VMAction::Ok
                    } else {
                        VMAction::Error("Data pointer < 0")
                    }
                }
                '+' => {
                    if self.data[self.dptr] < u8::MAX {
                        self.data[self.dptr] += 1;
                        VMAction::Ok
                    } else {
                        VMAction::Error("Data overflow")
                    }
                }
                '-' => {
                    if self.data[self.dptr] > 0 {
                        self.data[self.dptr] -= 1;
                        VMAction::Ok
                    } else {
                        VMAction::Error("Data underflow")
                    }
                }
                '.' => {
                    let mut buf = [0; 1];
                    buf[0] = self.data[self.dptr];
                    if let Ok(_) = self.output.write(&buf) {
                        VMAction::Ok
                    } else {
                        VMAction::Error("Unable to write output")
                    }
                }
                ',' => {
                    let mut buf = [0; 1];
                    if let Ok(()) = self.input.read_exact(&mut buf) {
                        self.data[self.dptr] = buf[0];
                        VMAction::Ok
                    } else {
                        VMAction::Error("Unable to read input")
                    }
                }
                '[' => {
                    if self.data[self.dptr] == 0 {
                        VMAction::JumpForward
                    } else {
                        VMAction::Ok
                    }
                }
                ']' => {
                    if self.data[self.dptr] != 0 {
                        VMAction::JumpBackward
                    } else {
                        VMAction::Ok
                    }
                }
                _ => VMAction::Ok,
            }
        } else {
            VMAction::Error("Invalid instruction pointer")
        }
    }
}