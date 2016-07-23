#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#[derive(Clone)]
enum InstructionSet {
    PSH (i32),
    ADD,
    POP,
    HLT
}

struct VM {
    running: bool,
    ip: usize,
    stack: Vec<i32>
}

fn eval(vm: &mut VM, inst: InstructionSet) {
    vm.ip += 1;
    match inst {
        InstructionSet::PSH(n) => vm.stack.push(n),
        InstructionSet::POP => {
            let value = vm.stack.pop();

            match value {
                Some(n) => println!("Popped: {}", n),
                None => println!("Error: Stack underflow at program line: {}", vm.ip)
            }
        },
        InstructionSet::ADD => {
            let a = vm.stack.pop();
            let b = vm.stack.pop();

            match (a, b) {
                (Some(a), Some(b)) => vm.stack.push(a + b),
                _ => {
                    println!("Error: Stack underflow at program line: {}", vm.ip);
                    vm.running = false;
                }
            };
        },
        InstructionSet::HLT => vm.running = false
    }
}

fn fetch(vm: &VM, program: &Vec<InstructionSet>) -> InstructionSet {
    match program.get(vm.ip) {
        None => InstructionSet::HLT,
        Some(next_inst) => next_inst.clone()
    }
}

fn main() {
    let mut vm = VM {
        running: true,
        ip: 0,
        stack: vec![]
    };

    let program: Vec<InstructionSet> = vec![
        InstructionSet::PSH(5),
        InstructionSet::PSH(6),
        InstructionSet::ADD,
        InstructionSet::POP,
        InstructionSet::HLT
    ];

    while vm.running {
        let instr = fetch(&vm, &program);
        eval(&mut vm, instr);
    }
}
