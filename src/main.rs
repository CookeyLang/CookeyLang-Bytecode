mod chunk;
mod values;
mod vm;

use chunk::{Chunk, OpCode, LineInfo};
use vm::VM;

fn main() {
    
    let mut chunk = Chunk::new();
    chunk.write_chunk(OpCode::Return, LineInfo::new(5, 6));
    
    let idx = chunk.add_constant(5f32);
    chunk.write_chunk(OpCode::Constant(idx), LineInfo::new(5, 5));

    let vm = VM::new(chunk);
    vm.interpret();
}
