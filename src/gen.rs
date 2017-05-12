use std::f32;
use std::cmp::min;
use std::io::Write;

use rand::{Rng, ThreadRng, thread_rng};

use config::Config;
use vm::{VM, VMResult};

const LEN_PENALTY_MODIFIER: f32 = 1000.0;
const OPERATIONS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

pub fn generate_program(cfg: Config) -> Result<String, &'static str> {
    let mut gen = ProgGen::new(cfg);
    gen.generate()
}

struct ProgGen {
    cfg: Config,
    prog: String,
    rng: ThreadRng,
}

impl ProgGen {
    fn new(cfg: Config) -> Self {
        ProgGen {
            cfg: cfg,
            prog: String::from(""),
            rng: thread_rng(),
        }
    }

    fn generate(&mut self) -> Result<String, &'static str> {
        for _ in 0..10 {
            self.mutate();
            println!("Prog: {}", self.prog);
            println!("Score: {}", self.score());
        }

        Ok(self.prog.clone())
    }

    fn score(&self) -> f32 {
        let mut score = 0.0;

        for rule in self.cfg.rules.iter() {
            let prog: Vec<u8> = self.prog.bytes().collect();
            let mut actual: Vec<u8> = Vec::new();

            {
                
                let mut vm = VM::new(prog, rule.input.as_bytes(), actual.by_ref());
                match vm.run() {
                    VMResult::Error(_) => {
                        score = f32::MAX;
                        continue;
                    },
                    VMResult::Ok => {}
                }
            }

            let expected: Vec<u8> = rule.output.bytes().collect();

            score += LEN_PENALTY_MODIFIER * (actual.len() as f32 - expected.len() as f32).abs();

            let len = min(actual.len(), expected.len());
            for i in 0..len {
                score += (actual[i] as f32 - expected[i] as f32).abs();
            }
        }

        score
    }

    fn mutate(&mut self) {
        match self.rng.gen_range(0, 3) {
            0 => self.mutate_insert(),
            1 => self.mutate_change(),
            2 => self.mutate_delete(),
            _ => {}
        }
    }

    fn mutate_insert(&mut self) {
        let idx = self.rng.gen_range(0, self.prog.len() + 1);
        let op = self.random_op();
        self.prog.insert(idx, op);
    }

    fn mutate_change(&mut self) {
        if self.prog.len() > 0 {
            let idx = self.rng.gen_range(0, self.prog.len());
            let op = self.random_op();
            self.prog.insert(idx, op);
            self.prog.remove(idx + 1);
        }
    }

    fn mutate_delete(&mut self) {
        if self.prog.len() > 1 {
            let idx = self.rng.gen_range(0, self.prog.len());
            if idx == self.prog.len() - 1 {
                self.prog.pop();
            } else {
                self.prog.remove(idx);
            }
        }
    }

    fn random_op(&mut self) -> char {
        self.rng.choose(&OPERATIONS).unwrap().clone()
    }
}
