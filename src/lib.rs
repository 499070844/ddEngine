use std::collections::HashMap;
mod compiler;


pub struct RegisterMachine {
    program_counter: usize,
    acc: Option<u64>,
    register: HashMap<u64, u64>
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
        }
    }
}

pub trait Instruction {
    fn execute(&self, vm: &mut RegisterMachine);
    fn offset_pc(&self, vm: &mut RegisterMachine, offset: isize) {
        // 0000 + 1111 -> 0 - 1
        // 0010 + 1111 -> 0001
        // 0111 + 1000 -> 1111
        // i -> -1 ,  u -> Max
        // u + i  ====> u
        // when offset (-)
        if offset.is_positive() {
            let p = vm.program_counter + offset as usize;
            if p < vm.program_counter {
                println!("Program counter overflow!");
                panic!();
            }
            vm.program_counter = p;
        } else {
            let p = vm.program_counter + offset as usize;
            if p > vm.program_counter {
                println!("Program counter overflow!");
                panic!();
            }
            vm.program_counter = p;
        }
    }

    fn is_hold(&self, vm: &RegisterMachine) -> bool {
        vm.acc.is_none()
    }
}

pub struct Lda {
    i: u64,
}

pub struct Sta {
    r: u64,
}

pub struct Add {
    r: u64,
}

pub struct Sub {
    r: u64,
}

pub struct Mult {
    r: u64,
}

pub struct Div {
    r: u64
}

pub struct AddImm {
    i: u64
}
pub struct SubImm {
    i: u64
}
pub struct MultImm {
    i: u64
}
pub struct DivImm {
    i: u64
}

pub struct LdaHold;

impl Instruction for Sta {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        vm.register.insert(self.r, vm.acc.unwrap());
    }
}

impl Instruction for Lda {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        vm.acc = Some(self.i);
    }
}

impl Instruction for LdaHold {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        vm.acc = None;
    }
}

impl Instruction for Add {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        match vm.register.get(&self.r) {
            Some(x) => vm.acc = vm.acc.map(|acc| acc + x),
            None => std::process::exit(1),
        }
    }
}

impl Instruction for Sub {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        match vm.register.get(&self.r) {
            Some(n) => vm.acc = vm.acc.map(|acc| acc - n),
            None => {
                println!("Do not have register r{}", self.r);
                std::process::exit(1);
            },
        }
    }
}
impl Instruction for Mult {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        match vm.register.get(&self.r) {
            Some(n) => vm.acc = vm.acc.map(|acc| acc * n),
            None => {
                println!("Do not have register r{}", self.r);
                std::process::exit(1);
            },
        }
    }
}
impl Instruction for Div {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        match vm.register.get(&self.r) {
            Some(n) => vm.acc = vm.acc.map(|acc| n / acc),
            None => {
                println!("Do not have register r{}", self.r);
                std::process::exit(1);
            },
        }
    }
}

impl Instruction for AddImm {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        vm.acc = vm.acc.map(|acc| self.i + acc);
    }
}

impl Instruction for SubImm {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        vm.acc = vm.acc.map(|acc| self.i - acc);
    }
}
impl Instruction for MultImm {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        vm.acc = vm.acc.map(|acc| self.i * acc);
    }
}
impl Instruction for DivImm {
    fn execute(&self, vm: &mut RegisterMachine) {
        self.offset_pc(vm, 1);
        vm.acc = vm.acc.map(|acc| self.i / acc);
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut vm = RegisterMachine::new();

        // Lda [1]
        let i_1 = Box::new(Lda { i: 1 });
        // Sta r0
        let i_2 = Box::new(Sta { r: 0 });
        // Lda [4]
        let i_3 = Box::new(Lda { i: 4 });
        // Add r0
        let i_4 = Box::new(Add { r: 0 });
        // Sta r0
        let i_5 = Box::new(Sta { r: 0 });
        // Lda [10]
        let i_6 = Box::new(Lda { i: 10 });
        // Mult r0
        let i_7 = Box::new(Mult { r: 0 });
        // Sta r0
        let i_8 = Box::new(Sta { r: 0 });
        // Lda [10]
        let i_9 = Box::new(Lda { i: 10 });
        // Div r0
        let i_10 = Box::new(Div { r: 0 });
        let mut tasks = Vec::<Box<dyn Instruction>>::new();
        tasks.push(i_1);
        tasks.push(i_2);
        tasks.push(i_3);
        tasks.push(i_4);
        tasks.push(i_5);
        tasks.push(i_6);
        tasks.push(i_7);
        tasks.push(i_8);
        tasks.push(i_9);
        tasks.push(i_10);

        vm.execute(&tasks);
        println!("result: acc: {}", vm.acc.unwrap());
    }

    #[test]
    fn pc_work() {
        let mut vm = RegisterMachine::new();
        struct Test();
        impl Instruction for Test {
            fn execute(&self, vm: &mut RegisterMachine) {
                self.offset_pc(vm, 10);
            }
        }
        let a = Test();
        a.offset_pc(&mut vm, 10);
        assert_eq!(10, vm.program_counter);
        a.offset_pc(&mut vm, -5);
        assert_eq!(5, vm.program_counter);
        a.offset_pc(&mut vm, 100);
        assert_eq!(105, vm.program_counter);
        a.offset_pc(&mut vm, -20);
        assert_eq!(85, vm.program_counter);
        // overflow!!! painc
        a.offset_pc(&mut vm, -100000);
    }
}
