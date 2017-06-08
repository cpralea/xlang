use common;
use cdata;

use super::llvm;


pub fn emit_llvm<'a>(block: &cdata::Block<'a>) -> common::Status<String> {
    common::Status {
        result: llvm::Emitter::new(block).emit(),
        error: None,
    }
}
