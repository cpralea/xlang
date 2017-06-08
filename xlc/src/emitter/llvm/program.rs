use ast;

use super::emitter;


impl<'a> emitter::Emitter<'a> {
    pub fn emit_program(&mut self) {
        self.emit_begin_program();
        self.emit_var_declarations();
        self.emit_statements();
        self.emit_end_program();
    }
}


impl<'a> emitter::Emitter<'a> {
    fn emit_begin_program(&mut self) {
        Self::ir(&mut self.ir, 0, "; Program entry point.");
        Self::ir(&mut self.ir, 0, "define i32 @main() {");
        Self::ir(&mut self.ir, 0, "entry:");
    }

    fn emit_statements(&mut self) {
        Self::ir(&mut self.ir, 1, "; Statements.");
        for step in self.block.steps.iter() {
            match *step.node.kind {
                ast::NodeKind::Expression { .. } => self.emit_expression(step),
                ast::NodeKind::Print { .. } => self.emit_print(step),
                ast::NodeKind::Assignment { .. } => self.emit_assignment(step),
                _ => unreachable!(),
            }
        }
    }

    fn emit_end_program(&mut self) {
        Self::ir(&mut self.ir, 1, "; Return.");
        Self::ir(&mut self.ir, 1, "ret i32 0");
        Self::ir(&mut self.ir, 0, "}");
        Self::ir(&mut self.ir, 0, "");
    }
}
