pub enum Operand {
    BoolLit { value: String },
    BoolVar { ir_id: String },
    IntLit { value: String },
    IntVar { ir_id: String },
    StrLit { value: String },
    StrVar { ir_id: String },
}
