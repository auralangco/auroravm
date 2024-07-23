use data::Data;
use env::Env;
use op::Opcode;

mod op;
mod data;
mod env;

fn main() {
    let env = Env::new(vec![Opcode::Push(60), Opcode::Push(9), Opcode::Dbg, Opcode::Add, Opcode::Retn], Default::default());

    let _ = dbg!(env.run());
}