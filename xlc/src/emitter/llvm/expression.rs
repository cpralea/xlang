use ast;
use cdata;

use super::emitter;
use super::operand;


impl<'a> emitter::Emitter<'a> {
    pub fn emit_expression(&mut self, step: &'a cdata::Step<'a>) {
        match *step.node.kind {
            ast::NodeKind::Expression {
                ref boolean,
                ref identifier,
                ref integer,
                ref string,
                ref operator,
                ..
            } => {
                let (xl_boolval, xl_id, xl_intval, xl_strval, xl_op) =
                    (boolean, identifier, integer, string, operator);
                if let Some(ref xl_val) = *xl_boolval {
                    self.push_bollit(xl_val);
                }
                if let Some(ref xl_id) = *xl_id {
                    self.push_var(step, xl_id);
                }
                if let Some(ref xl_val) = *xl_intval {
                    self.push_intlit(xl_val);
                }
                if let Some(ref xl_val) = *xl_strval {
                    self.push_strlit(xl_val);
                }
                if let Some(ref xl_op) = *xl_op {
                    self.emit_op(xl_op);
                }
            }
            _ => unreachable!(),
        }
    }
}


impl<'a> emitter::Emitter<'a> {
    fn emit_op(&mut self, xl_op: &String) {
        assert!(self.stack.len() >= 2);
        let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        let (ir_leftid, ir_rightid) = (self.get_ir_id_by_op(left), self.get_ir_id_by_op(right));
        match xl_op.as_str() {
            "+" => self.emit_op_add(ir_leftid, ir_rightid),
            "-" => self.emit_op_sub(ir_leftid, ir_rightid),
            "*" => self.emit_op_mul(ir_leftid, ir_rightid),
            "/" => self.emit_op_div(ir_leftid, ir_rightid),
            "||" => self.emit_op_or(ir_leftid, ir_rightid),
            "&&" => self.emit_op_and(ir_leftid, ir_rightid),
            _ => unreachable!(),
        }
    }

    pub fn emit_op_add(&mut self, ir_leftid: String, ir_rightid: String) {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = add nsw i64 {}, {}", ir_tmpid, ir_leftid, ir_rightid).as_str());
        self.stack.push(operand::Operand::IntLit { value: ir_tmpid });
    }

    pub fn emit_op_sub(&mut self, ir_leftid: String, ir_rightid: String) {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = sub nsw i64 {}, {}", ir_tmpid, ir_leftid, ir_rightid).as_str());
        self.stack.push(operand::Operand::IntLit { value: ir_tmpid });
    }

    pub fn emit_op_mul(&mut self, ir_leftid: String, ir_rightid: String) {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = mul nsw i64 {}, {}", ir_tmpid, ir_leftid, ir_rightid).as_str());
        self.stack.push(operand::Operand::IntLit { value: ir_tmpid });
    }

    pub fn emit_op_div(&mut self, ir_leftid: String, ir_rightid: String) {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = sdiv i64 {}, {}", ir_tmpid, ir_leftid, ir_rightid).as_str());
        self.stack.push(operand::Operand::IntLit { value: ir_tmpid });
    }

    pub fn emit_op_or(&mut self, ir_leftid: String, ir_rightid: String) {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = or i8 {}, {}", ir_tmpid, ir_leftid, ir_rightid).as_str());
        self.stack.push(operand::Operand::BoolLit { value: ir_tmpid });
    }

    pub fn emit_op_and(&mut self, ir_leftid: String, ir_rightid: String) {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = and i8 {}, {}", ir_tmpid, ir_leftid, ir_rightid).as_str());
        self.stack.push(operand::Operand::BoolLit { value: ir_tmpid });
    }

    pub fn get_ir_id_by_op(&mut self, op: operand::Operand) -> String {
        match op {
            operand::Operand::BoolLit { ref value, .. } => value.clone(),
            operand::Operand::BoolVar { ref ir_id, .. } => self.emit_load_booltmpid(ir_id),
            operand::Operand::IntLit { ref value, .. } => value.clone(),
            operand::Operand::IntVar { ref ir_id, .. } => self.emit_load_inttmpid(ir_id),
            _ => unreachable!(),
        }
    }

    fn push_var(&mut self, step: &'a cdata::Step<'a>, xl_id: &String) {
        let ir_id = Self::get_ir_id_by_xl_id(&self.var_decls, xl_id).clone();
        match step.kind {
            cdata::StepKind::Bool => self.stack.push(operand::Operand::BoolVar { ir_id: ir_id }),
            cdata::StepKind::Int => self.stack.push(operand::Operand::IntVar { ir_id: ir_id }),
            cdata::StepKind::Str => self.stack.push(operand::Operand::StrVar { ir_id: ir_id }),
            _ => unreachable!(),
        }
    }

    fn push_bollit(&mut self, xl_val: &bool) {
        let ir_val = match *xl_val {
                true => "1",
                false => "0",
            }
            .to_string();
        self.stack.push(operand::Operand::BoolLit { value: ir_val });
    }

    fn push_intlit(&mut self, xl_val: &i64) {
        let ir_val = xl_val.to_string();
        self.stack.push(operand::Operand::IntLit { value: ir_val });
    }

    fn push_strlit(&mut self, xl_val: &String) {
        let ir_id = Self::get_ir_id_by_xl_val(&self.str_decls, xl_val).clone();
        self.stack.push(operand::Operand::StrLit { value: ir_id });
    }
}
