use ast;
use cdata;

use super::emitter;
use super::vardecl;

impl<'a> emitter::Emitter<'a> {
    pub fn emit_var_declarations(&mut self) {
        self.build_var_decls();
        Self::ir(&mut self.ir, 1, "; Variables.");
        for var_decl in self.var_decls.iter() {
            Self::ir(
                &mut self.ir,
                1,
                format!("{} = alloca {}, align 8", var_decl.ir_id, var_decl.ir_type).as_str(),
            )
        }
    }
}


impl<'a> emitter::Emitter<'a> {
    fn build_var_decls(&mut self) {
        for step in self.block.steps.iter() {
            if let ast::NodeKind::Assignment { ref identifier, .. } = *step.node.kind {
                let xl_id = identifier;
                if let Some(ref xl_id) = *xl_id {
                    if Self::find_var_decl_by_xl_id(&self.var_decls, xl_id).is_none() {
                        let ir_id = format!("%{}", xl_id);
                        let ir_type = String::from(match step.kind {
                            cdata::StepKind::Bool => "i8",
                            cdata::StepKind::Int => "i64",
                            cdata::StepKind::Str => "i8*",
                            _ => unreachable!(),
                        });
                        self.var_decls.push(
                            vardecl::VarDecl::from(xl_id).with_id(ir_id).with_type(
                                ir_type,
                            ),
                        );
                    }
                }
            }
        }
    }
}
