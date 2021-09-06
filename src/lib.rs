mod interpreter;
mod compiler;
use std::collections::HashMap;


use interpreter::{
    instruction::Instruction,
    context::Context as InterpreterContext,
};


pub struct RegisterMachine {
    program_counter: usize,
    acc: Option<u64>,
    register: HashMap<u64, u64>,
    context: InterpreterContext,
}

impl RegisterMachine {
    pub fn execute(&mut self, instruction: &Vec<Box<dyn Instruction>>) {
        loop {
            if self.program_counter >= instruction.len() {
                println!("所有指令执行完毕");
                // std::process::exit(0);
                break;
            } 

            instruction[self.program_counter].execute(self);
        }
    }

    pub fn new() -> RegisterMachine {
        RegisterMachine {
            program_counter: 0,
            acc: None,
            register: HashMap::new(),
            context: InterpreterContext {},
        }
    }
}




