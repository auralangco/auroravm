use data::Data;
use env::Env;
use op::Opcode;

mod op;
mod data;
mod env;
mod ntv;

fn main() {
    let env = Env::new_no_retn(vec![Opcode::Push(60), Opcode::Push(9), Opcode::Add, Opcode::Natvnr("println"), Opcode::Retn], Default::default());

    let _ = dbg!(env.run());
}