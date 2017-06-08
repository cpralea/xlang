use ast;
use cdata;

use super::emitter;
use super::operand;


impl<'a> emitter::Emitter<'a> {
    pub fn emit_print(&mut self, step: &'a cdata::Step<'a>) {
        match *step.node.kind {
            ast::NodeKind::Print { .. } => {
                assert!(self.stack.len() == 1);
                match self.stack.pop() {
                    Some(operand::Operand::BoolLit { ref value, .. }) => {
                        self.emit_print_boollit(value);
                    }
                    Some(operand::Operand::BoolVar { ref ir_id, .. }) => {
                        self.emit_print_boolvar(ir_id);
                    }
                    Some(operand::Operand::IntLit { ref value, .. }) => {
                        self.emit_print_intlit(value);
                    }
                    Some(operand::Operand::IntVar { ref ir_id, .. }) => {
                        self.emit_print_intvar(ir_id);
                    }
                    Some(operand::Operand::StrLit { ref value, .. }) => {
                        self.emit_print_strlit(value);
                    }
                    Some(operand::Operand::StrVar { ref ir_id, .. }) => {
                        self.emit_print_strvar(ir_id);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}


impl<'a> emitter::Emitter<'a> {
    fn emit_print_boollit(&mut self, ir_id: &String) {
        self.emit_print_bool(ir_id);
    }

    fn emit_print_boolvar(&mut self, ir_id: &String) {
        let ir_tmpid = &self.emit_load_booltmpid(ir_id);
        self.emit_print_bool(ir_tmpid);
    }

    fn emit_print_intlit(&mut self, ir_id: &String) {
        self.emit_print_int(ir_id);
    }

    fn emit_print_intvar(&mut self, ir_id: &String) {
        let ir_tmpid = &self.emit_load_inttmpid(ir_id);
        self.emit_print_int(ir_tmpid);
    }

    fn emit_print_strlit(&mut self, ir_id: &String) {
        let ir_id = &Self::get_decorated_str_ir_id(Self::get_str_decl_by_ir_id(&self.str_decls,
                                                                               ir_id));
        self.emit_print_str(ir_id);
    }

    fn emit_print_strvar(&mut self, ir_id: &String) {
        let ir_tmpid = &self.emit_load_strtmpid(ir_id);
        self.emit_print_str(ir_tmpid);
    }

    fn emit_print_bool(&mut self, ir_id: &String) {
        Self::ir(&mut self.ir, 1, format!("call void @__xlrt_print_bool(i8 {})", ir_id).as_str());
    }

    fn emit_print_int(&mut self, ir_id: &String) {
        Self::ir(&mut self.ir, 1, format!("call void @__xlrt_print_int(i64 {})", ir_id).as_str());
    }

    fn emit_print_str(&mut self, ir_id: &String) {
        Self::ir(&mut self.ir, 1, format!("call void @__xlrt_print_str(i8* {})", ir_id).as_str());
    }
}
