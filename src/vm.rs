use std::io::{Read, Write, stdin, stdout};

const DATA_SIZE: usize = 30000;

pub struct VM {
    prog: Vec<u8>,
    iptr: usize,
    dptr: usize,
    data: [u8; DATA_SIZE],
}

enum VMAction {
    EOF,
    Error(&'static str),
    JumpForward,
    JumpBackward,
    Ok,
}

impl VM {
    pub fn new(prog: Vec<u8>) -> Self {
        VM {
            prog: prog,
            iptr: 0,
            dptr: 0,
            data: [0; DATA_SIZE],
        }
    }

    pub fn run(&mut self) {
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
                            None => panic!("No matching ]"),
                        }

                        self.iptr += 1;
                    }
                }
                VMAction::JumpBackward => {
                    let mut count = 1;
                    while count > 0 {
                        self.iptr -= 1;

                        match self.prog.get(self.iptr) {
                            Some(byte) => {
                                match byte.clone() as char {
                                    ']' => count += 1,
                                    '[' => count -= 1,
                                    _ => {}
                                }
                            }
                            None => panic!("No matching ["),
                        }
                    }

                    self.iptr += 1;
                }
                VMAction::Error(e) => panic!(e),
                VMAction::EOF => break,
            }
        }
    }

    fn step(&mut self) -> VMAction {
        if self.iptr >= self.prog.len() {
            return VMAction::EOF;
        }

        if let Some(byte) = self.prog.get(self.iptr) {
            match byte.clone() as char {
                '>' => {
                    self.dptr += 1;
                    VMAction::Ok
                }
                '<' => {
                    self.dptr -= 1;
                    VMAction::Ok
                }
                '+' => {
                    self.data[self.dptr] += 1;
                    VMAction::Ok
                }
                '-' => {
                    self.data[self.dptr] -= 1;
                    VMAction::Ok
                }
                '.' => {
                    let mut buf = [0; 1];
                    buf[0] = self.data[self.dptr];
                    stdout().write(&buf).unwrap();
                    VMAction::Ok
                }
                ',' => {
                    let mut buf = [0; 1];
                    stdin().take(1).read(&mut buf).unwrap();
                    self.data[self.dptr] = buf[0];
                    VMAction::Ok
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