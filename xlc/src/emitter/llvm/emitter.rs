use cdata;

use super::operand;
use super::strdecl;
use super::vardecl;


pub struct Emitter<'a> {
    pub block: &'a cdata::Block<'a>,
    pub ir: String,
    pub str_decls: Vec<strdecl::StrDecl<'a>>,
    pub var_decls: Vec<vardecl::VarDecl<'a>>,
    pub stack: Vec<operand::Operand>,
    pub strlit_id: usize,
    pub tmpvar_id: usize,
}


impl<'a> Emitter<'a> {
    pub fn new(block: &'a cdata::Block<'a>) -> Self {
        Emitter {
            block: block,
            ir: String::new(),
            str_decls: Vec::new(),
            var_decls: Vec::new(),
            stack: Vec::new(),
            strlit_id: 0,
            tmpvar_id: 0,
        }
    }

    pub fn emit(mut self) -> String {
        self.emit_str_literals();
        self.emit_program();
        self.emit_xlrt_declarations();
        self.ir
    }
}
