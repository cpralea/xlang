use ast;

use super::emitter;
use super::strdecl;


impl<'a> emitter::Emitter<'a> {
    pub fn emit_str_literals(&mut self) {
        self.build_str_decls();
        Self::ir(&mut self.ir, 0, "; String literals.");
        for str_decl in self.str_decls.iter() {
            Self::ir(
                &mut self.ir,
                0,
                format!(
                    "{} = private unnamed_addr constant [{} x i8] c\"{}\", align 1",
                    str_decl.ir_id,
                    str_decl.ir_len,
                    str_decl.ir_val
                ).as_str(),
            )
        }
        Self::ir(&mut self.ir, 0, "");
    }
}


impl<'a> emitter::Emitter<'a> {
    fn build_str_decls(&mut self) {
        for step in self.block.steps.iter() {
            if let ast::NodeKind::Expression { ref string, .. } = *step.node.kind {
                let xl_val = string;
                if let Some(ref xl_val) = *xl_val {
                    if Self::find_str_decl_by_xl_val(&self.str_decls, xl_val).is_none() {
                        let ir_id = format!("@{}", Self::get_next_id(&mut self.strlit_id));
                        self.str_decls.push(strdecl::StrDecl::from(xl_val).with_id(ir_id));
                    }
                }
            }
        }
    }
}
