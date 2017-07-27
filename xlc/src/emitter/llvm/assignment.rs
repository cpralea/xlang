use ast;
use cdata;

use super::emitter;
use super::operand;


impl<'a> emitter::Emitter<'a> {
    pub fn emit_assignment(&mut self, step: &'a cdata::Step<'a>) {
        match *step.node.kind {
            ast::NodeKind::Assignment { ref identifier, .. } => {
                assert!(self.stack.len() == 1);
                let xl_id = identifier.as_ref().unwrap();
                match self.stack.pop() {
                    Some(operand::Operand::BoolLit { ref value, .. }) => {
                        self.emit_assignment_boollit(value, xl_id);
                    }
                    Some(operand::Operand::BoolVar { ref ir_id, .. }) => {
                        self.emit_assignment_boolvar(ir_id, xl_id);
                    }
                    Some(operand::Operand::IntLit { ref value, .. }) => {
                        self.emit_assignment_intlit(value, xl_id);
                    }
                    Some(operand::Operand::IntVar { ref ir_id, .. }) => {
                        self.emit_assignment_intvar(ir_id, xl_id);
                    }
                    Some(operand::Operand::StrLit { ref value, .. }) => {
                        self.emit_assignment_strlit(value, xl_id);
                    }
                    Some(operand::Operand::StrVar { ref ir_id, .. }) => {
                        self.emit_assignment_strvar(ir_id, xl_id);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}


impl<'a> emitter::Emitter<'a> {
    fn emit_assignment_boollit(&mut self, ir_srcid: &String, xl_dstid: &String) {
        self.emit_assignment_bool(ir_srcid, xl_dstid);
    }

    fn emit_assignment_boolvar(&mut self, ir_srcid: &String, xl_dstid: &String) {
        let ir_tmpid = &self.emit_load_booltmpid(ir_srcid);
        self.emit_assignment_bool(ir_tmpid, xl_dstid);
    }

    fn emit_assignment_intlit(&mut self, ir_srcid: &String, xl_dstid: &String) {
        self.emit_assignment_int(ir_srcid, xl_dstid);
    }

    fn emit_assignment_intvar(&mut self, ir_srcid: &String, xl_dstid: &String) {
        let ir_tmpid = &self.emit_load_inttmpid(ir_srcid);
        self.emit_assignment_int(ir_tmpid, xl_dstid);
    }

    fn emit_assignment_strlit(&mut self, ir_srcid: &String, xl_dstid: &String) {
        let ir_srcid =
            &Self::get_decorated_str_ir_id(Self::get_str_decl_by_ir_id(&self.str_decls, ir_srcid));
        self.emit_assignment_str(ir_srcid, xl_dstid);
    }

    fn emit_assignment_strvar(&mut self, ir_srcid: &String, xl_dstid: &String) {
        let ir_tmpid = &self.emit_load_strtmpid(ir_srcid);
        self.emit_assignment_str(ir_tmpid, xl_dstid);
    }

    fn emit_assignment_bool(&mut self, ir_srcid: &String, xl_dstvar: &String) {
        let ir_dstid = Self::get_ir_id_by_xl_id(&self.var_decls, xl_dstvar);
        Self::ir(
            &mut self.ir,
            1,
            format!("store i8 {}, i8* {}, align 1", ir_srcid, ir_dstid).as_str(),
        );
    }

    fn emit_assignment_int(&mut self, ir_srcid: &String, xl_dstvar: &String) {
        let ir_dstid = Self::get_ir_id_by_xl_id(&self.var_decls, xl_dstvar);
        Self::ir(
            &mut self.ir,
            1,
            format!("store i64 {}, i64* {}, align 8", ir_srcid, ir_dstid).as_str(),
        );
    }

    fn emit_assignment_str(&mut self, ir_srcid: &String, xl_dstvar: &String) {
        let ir_dstid = Self::get_ir_id_by_xl_id(&self.var_decls, xl_dstvar);
        Self::ir(
            &mut self.ir,
            1,
            format!("store i8* {}, i8** {}, align 8", ir_srcid, ir_dstid).as_str(),
        );
    }
}
