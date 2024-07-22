use crate::data::Data;

#[derive(Debug, Clone)]
pub enum Opcode {
    /// No operation, the program state still the same
    Nop,
    /// Finishes the execution of the current execution stack
    Retn,
    /// Debug the current stack state
    Dbg,
    /// Pushes a value to the stack
    Push(i64),
    /// Pops a value from the stack
    Pop,
    /// Sums the two values at the top of the stack
    Add,
}