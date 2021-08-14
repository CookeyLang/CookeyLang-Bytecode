use crate::values::{Value, ValueArray};

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(usize), // pointer to constants array
    Return,
}

#[derive(Debug, Clone, Copy)]
pub struct LineInfo {
    line: i32,
    col: i32
}

impl LineInfo {
    pub fn new(line: i32, col: i32) -> Self {
        Self { line, col }
    }
}


#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<LineInfo>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write_chunk(&mut self, op: OpCode, line: LineInfo) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, val: Value) -> usize {
        self.constants.write_array(val)
    }

    // debug
    pub fn debug_chunk(&self, name: &str) {
        println!("== {} ==", name);

        for (i, op) in self.code.iter().enumerate() {
            print!("{:0>4}\t", i);
            print!("{:0>2}:{:0>2}\t", self.lines[i].line, self.lines[i].col);
            self.debug_instruction(op);
            println!();
        }
    }

    pub fn debug_instruction(&self, op: &OpCode) {
        match op {
            OpCode::Return => print!("RETURN"),
            OpCode::Constant(x) => print!("CONSTANT => {}", self.constants.get(*x))
        }
    }
}
