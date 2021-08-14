use crate::{chunk::{Chunk, OpCode}, values::Value};

const MAX_STACK: usize = 256;
const DEBUG: bool = true;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}

pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk: chunk.clone(), stack: Vec::with_capacity(MAX_STACK) }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    
    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn interpret(&mut self) -> InterpretResult {
        for instruction in &self.chunk.code {
            if DEBUG {
                println!("\n\n== stack ==");
                for i in &self.stack {
                    print!("[ ");
                    print!("{}", i);
                    print!(" ]");
                }
                println!();

                print!("== type: ");
                self.chunk.debug_instruction(instruction);
                println!();
            }

            match instruction {
                OpCode::Return => {
                    
                },
                OpCode::Constant(pointer) => {
                    let val = self.chunk.constants.get(*pointer);
                    self.push(val);
                    println!("{}", val);
                }
            }
        }

        return InterpretResult::Ok;
    }
}