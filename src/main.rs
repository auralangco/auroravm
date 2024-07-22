use data::Data;
use op::Opcode;

mod op;
mod data;

fn main() {
    let mut stack: Vec<Data> = vec![];
    let mut program = vec![Opcode::Nop, Opcode::Push(1), Opcode::Push(2), Opcode::Dbg, Opcode::Add, Opcode::Pop, Opcode::Retn];
    let mut pc: usize = 0;

    loop {
        let oldpc = pc;
        match step(pc, stack, program) {
            Ok(state) => {
                pc = state.0;
                stack = state.1;
                program = state.2;

                if pc == oldpc { 
                    dbg!(stack);
                    break; 
                }
            },
            Err(()) => { break; }
        }
    }
}

fn step(pc: usize, mut stack: Vec<Data>, program: Vec<Opcode>) -> Result<(usize, Vec<Data>, Vec<Opcode>), ()> {
    if pc >= program.len() { return Ok((pc, stack, program)) }

    match program[pc] {
        Opcode::Nop => Ok((pc+1, stack, program)),
        Opcode::Retn => Ok((pc, stack, program)),
        Opcode::Dbg => {
            println!("DBG\n\tpc: {}\n\tstack: {:?}", pc, stack);

            Ok((pc+1, stack, program))
        }
        Opcode::Push(i) => Ok((pc+1, { stack.push(Data::Value(i)); stack }, program)),
        Opcode::Pop => {
            let top = stack.pop();

            match top {
                Some(top) => {
                    println!("Pop: {:?}", top);
                    Ok((pc+1, stack, program))
                },
                None => {
                    eprintln!("Pop empty stack");
                    Err(())
                },
            }
        },
        Opcode::Add => {
            let b = stack.pop();
            let a = stack.pop();

            match (a, b) {
                (Some(Data::Value(a)), Some(Data::Value(b))) => {
                    stack.push(Data::Value(a + b));
                    Ok((pc+1, stack, program))
                },
                (None, _) | (_, None) => {
                    eprintln!("[Add] stack must have at least 2 elements");
                    Err(())
                },
                (_, _) => {
                    eprintln!("[Add] both top elements in the stack must be numeric values");
                    Err(())
                }
            }
        },
    }
}