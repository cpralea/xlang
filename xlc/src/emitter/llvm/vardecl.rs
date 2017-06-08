pub struct VarDecl<'a> {
    pub ir_id: String,
    pub ir_type: String,
    pub xl_id: &'a String,
}


impl<'a> VarDecl<'a> {
    pub fn from(xl_id: &'a String) -> Self {
        VarDecl {
            ir_id: String::from("N/A"),
            ir_type: String::from("N/A"),
            xl_id: xl_id,
        }
    }
    pub fn with_id(mut self, ir_id: String) -> Self {
        self.ir_id = ir_id;
        self
    }
    pub fn with_type(mut self, ir_type: String) -> Self {
        self.ir_type = ir_type;
        self
    }
}
