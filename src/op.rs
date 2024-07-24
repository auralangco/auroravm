use crate::data::Data;

#[derive(Debug, Clone)]
pub enum Opcode<'a> {
    /// No operation, the program state still the same
    Nop,
    /// Finishes the execution of the current execution stack
    Retn,
    /// Debug the current stack state
    Dbg,
    /// Pushes a value to the stack
    Push(Data),
    /// Pops a value from the stack
    Pop,
    /// Sums the two values at the top of the stack
    Add,
    /// Calls a native function with the given name
    Natv(&'a str),
    /// Calls a native function with the given name that won't return
    Natvnr(&'a str),
}