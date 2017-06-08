use std::ops::Add;


pub struct StrDecl<'a> {
    pub ir_id: String,
    pub ir_val: String,
    pub ir_len: usize,
    pub xl_val: &'a String,
}


impl<'a> StrDecl<'a> {
    pub fn from(xl_val: &'a String) -> Self {
        let ir_val = Self::ir_val(xl_val);
        let ir_len = Self::ir_len(&ir_val);
        StrDecl {
            ir_id: String::from("N/A"),
            ir_val: ir_val,
            ir_len: ir_len,
            xl_val: xl_val,
        }
    }
    pub fn with_id(mut self, ir_id: String) -> Self {
        self.ir_id = ir_id;
        self
    }

    fn ir_val(xl_val: &String) -> String {
        xl_val
            .replace("\\n", "\\0A")
            .replace("\\r", "\\0D")
            .replace("\\t", "\\09")
            .replace("\\\"", "\\22")
            .replace("\\\\", "\\5C")
            .add("\\00")
    }
    fn ir_len(ir_val: &String) -> usize {
        ir_val.len() - 2 * ir_val.matches('\\').count()
    }
}
