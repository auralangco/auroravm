//! Run environments are isolated runs of a aurora function
//! 
//! This can represent both the entrypoint of the program or a execution of the `call` operation

use std::collections::HashMap;

use crate::{data::Data, op::Opcode};

pub struct Env {
    /// The current instruction being executed
    pub program_counter: usize,
    /// The program that this environment will run
    pub program: Vec<Opcode>,
    /// The data stack
    pub stack: Vec<Data>,
    /// The heap with the environment variables
    pub heap: HashMap<String, Data>, 
    /// A flag to indicate if the environment should return a value
    pub returns: bool,
}

pub enum Control {
    Next,
    Break
}

impl Env {
    /// Creates a new environment with the given program and heap that will return a value
    /// 
    /// The program counter starts from 0 and the stack is empty
    pub fn new(program: Vec<Opcode>, heap: HashMap<String, Data>) -> Self {
        Self {
            program,
            heap,
            program_counter: 0,
            stack: Default::default(),
            returns: true
        }
    }

    /// Creates a new run environment with the given program and heap the won't return a value
    pub fn new_no_retn(program: Vec<Opcode>, heap: HashMap<String, Data>) -> Self {
        Self{ 
            returns: false, 
            ..Self::new(program, heap)
        }
    }

    pub fn run(mut self) -> Result<Option<Data>, ()> {
        loop {
            if let Control::Break = self.step()? {
                return Ok(
                    if self.returns {
                        self.stack.pop()
                    } else {
                        None
                    }
                )
            }
        }
    }

    fn step(&mut self) -> Result<Control, ()> {
        match self.program[self.program_counter] {
            Opcode::Nop => {
                self.program_counter += 1;
                Ok(Control::Next)
            },
            Opcode::Retn => {
                Ok(Control::Break)
            },
            Opcode::Dbg => {
                println!("pc: {}\nstack: {:?}", self.program_counter, self.stack);
                self.program_counter += 1;
                Ok(Control::Next)
            },
            Opcode::Push(data) => {
                self.stack.push(Data(data));
                self.program_counter += 1;
                Ok(Control::Next)
            },
            Opcode::Pop => match self.stack.pop() {
                Some(data) => {
                    println!("{:?}", data);
                    self.program_counter += 1;
                    Ok(Control::Next)
                },
                None => Err(())
            },
            Opcode::Add => {
                let b = self.stack.pop().ok_or(())?;
                let a = self.stack.pop().ok_or(())?;

                self.stack.push(Data(a.0 + b.0));
                self.program_counter += 1;
                Ok(Control::Next)
            },
        }
    }
}