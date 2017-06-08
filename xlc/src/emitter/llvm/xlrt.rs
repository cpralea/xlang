use super::emitter;


impl<'a> emitter::Emitter<'a> {
    pub fn emit_xlrt_declarations(&mut self) {
        Self::ir(&mut self.ir, 0, "; X language runtime symbols.");
        Self::ir(&mut self.ir, 0, "declare void @__xlrt_print_bool(i8)");
        Self::ir(&mut self.ir, 0, "declare void @__xlrt_print_int(i64)");
        Self::ir(&mut self.ir, 0, "declare void @__xlrt_print_str(i8*)");
    }
}
