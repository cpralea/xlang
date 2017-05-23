use common;
use ast;
use cdata;

use std::ops::Add;


pub fn emit_llvm<'a>(cdata: &cdata::CompilerData<'a>) -> common::Status<String> {
    common::Status { result: Emitter::new(cdata).emit(), error: None }
}


struct Emitter<'a> {
    cdata: &'a cdata::CompilerData<'a>,
    ir: String,
    str_decls: Vec<StrDeclaration<'a>>,
    var_decls: Vec<VarDeclaration<'a>>,
    stack: Vec<Operand>,
    strlit_id: usize,
    tmpvar_id: usize,
}


struct StrDeclaration<'a> {
    ir_id: String,
    ir_val: String,
    ir_len: usize,
    xl_val: &'a String,
}
struct VarDeclaration<'a> {
    ir_id: String,
    ir_type: String,
    xl_id: &'a String,
}
enum Operand {
    BoolLit { value: String },
    IntLit  { value: String },
    StrLit  { value: String },
    BoolVar { ir_id: String },
    IntVar  { ir_id: String },
    StrVar  { ir_id: String },
}


impl<'a> Emitter<'a> {

    fn emit(mut self) -> String {
        self.emit_str_literals();
        self.emit_program();
        self.emit_xlrt_declarations();
        self.ir
    }

    fn emit_str_literals(&mut self) {
        self.build_str_decls();
        Self::ir(&mut self.ir, 0, "; String literals.");
        for str_decl in self.str_decls.iter() {
            Self::ir(&mut self.ir, 0, format!(
                "{} = private unnamed_addr constant [{} x i8] c\"{}\", align 1"
            , str_decl.ir_id, str_decl.ir_len, str_decl.ir_val).as_str())
        }
        Self::ir(&mut self.ir, 0, "");
    }

    fn emit_program(&mut self) {
        self.emit_begin_program();
        self.emit_var_declarations();
        self.emit_statements();
        self.emit_end_program();
    }

    fn emit_xlrt_declarations(&mut self) {
        Self::ir(&mut self.ir, 0, "; X language runtime symbols.");
        Self::ir(&mut self.ir, 0, "declare void @__xlrt_print_bool(i8)");
        Self::ir(&mut self.ir, 0, "declare void @__xlrt_print_int(i64)");
        Self::ir(&mut self.ir, 0, "declare void @__xlrt_print_str(i8*)");
    }

    fn emit_begin_program(&mut self) {
        Self::ir(&mut self.ir, 0, "; Program entry point.");
        Self::ir(&mut self.ir, 0, "define i32 @main() {");
        Self::ir(&mut self.ir, 0, "entry:");
    }

    fn emit_var_declarations(&mut self) {
        self.build_var_decls();
        Self::ir(&mut self.ir, 1, "; Variables.");
        for var_decl in self.var_decls.iter() {
            Self::ir(&mut self.ir, 1, format!(
                "{} = alloca {}, align 8"
            , var_decl.ir_id, var_decl.ir_type).as_str())
        }
    }

    fn emit_statements(&mut self) {
        Self::ir(&mut self.ir, 1, "; Statements.");
        for step in self.cdata.steps.iter() {
            match *step.node.kind {
                ast::NodeKind::Expression {..} => self.push_expression(step),
                ast::NodeKind::Print {..}      => self.emit_print(step),
                ast::NodeKind::Assignment {..} => self.emit_assignment(step),
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

    fn emit_print(&mut self, step: &'a cdata::Step<'a>) {
        match *step.node.kind { ast::NodeKind::Print {..} => {
            assert!(self.stack.len() == 1);
            match self.stack.pop() {
                Some(Operand::BoolLit { ref value, .. }) =>
                    self.emit_print_boollit(value),
                Some(Operand::BoolVar { ref ir_id, .. }) =>
                    self.emit_print_boolvar(ir_id),
                Some(Operand::IntLit { ref value, .. }) =>
                    self.emit_print_intlit(value),
                Some(Operand::IntVar { ref ir_id, .. }) =>
                    self.emit_print_intvar(ir_id),
                Some(Operand::StrLit { ref value, .. }) =>
                    self.emit_print_strlit(value),
                Some(Operand::StrVar { ref ir_id, .. }) =>
                    self.emit_print_strvar(ir_id),
        _ => unreachable!(), }}, _ => unreachable!(), }
    }

    fn emit_assignment(&mut self, step: &'a cdata::Step<'a>) {
        match *step.node.kind { ast::NodeKind::Assignment { ref identifier, .. } => {
            assert!(self.stack.len() == 1);
            let xl_id = identifier.as_ref().unwrap();
            match self.stack.pop() {
                Some(Operand::BoolLit { ref value, .. }) =>
                    self.emit_assignment_boollit(value, xl_id),
                Some(Operand::BoolVar { ref ir_id, .. }) =>
                    self.emit_assignment_boolvar(ir_id, xl_id),
                Some(Operand::IntLit { ref value, .. }) =>
                    self.emit_assignment_intlit(value, xl_id),
                Some(Operand::IntVar { ref ir_id, .. }) =>
                    self.emit_assignment_intvar(ir_id, xl_id),
                Some(Operand::StrLit { ref value, .. }) =>
                    self.emit_assignment_strlit(value, xl_id),
                Some(Operand::StrVar { ref ir_id, .. }) =>
                    self.emit_assignment_strvar(ir_id, xl_id),
        _ => unreachable!(), }}, _ => unreachable!(), }
    }

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
        let ir_id = &Self::get_decorated_str_ir_id(
                        Self::get_str_decl_by_ir_id(&self.str_decls, ir_id));
        self.emit_print_str(ir_id);
    }

    fn emit_print_strvar(&mut self, ir_id: &String) {
        let ir_tmpid = &self.emit_load_strtmpid(ir_id);
        self.emit_print_str(ir_tmpid);
    }

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
        let ir_srcid = &Self::get_decorated_str_ir_id(
                           Self::get_str_decl_by_ir_id(&self.str_decls, ir_srcid));
        self.emit_assignment_str(ir_srcid, xl_dstid);
    }

    fn emit_assignment_strvar(&mut self, ir_srcid: &String, xl_dstid: &String) {
        let ir_tmpid = &self.emit_load_strtmpid(ir_srcid);
        self.emit_assignment_str(ir_tmpid, xl_dstid);
    }

    fn emit_print_bool(&mut self, ir_id: &String) {
        Self::ir(&mut self.ir, 1, format!(
            "call void @__xlrt_print_bool(i8 {})"
        , ir_id).as_str());
    }

    fn emit_print_int(&mut self, ir_id: &String) {
        Self::ir(&mut self.ir, 1, format!(
            "call void @__xlrt_print_int(i64 {})"
        , ir_id).as_str());
    }

    fn emit_print_str(&mut self, ir_id: &String) {
        Self::ir(&mut self.ir, 1, format!(
            "call void @__xlrt_print_str(i8* {})"
        , ir_id).as_str());
    }

    fn emit_assignment_bool(&mut self, ir_srcid: &String, xl_dstvar: &String) {
        let ir_dstid = Self::get_ir_id_by_xl_id(&self.var_decls, xl_dstvar);
        Self::ir(&mut self.ir, 1, format!(
            "store i8 {}, i8* {}, align 1"
        , ir_srcid, ir_dstid).as_str());
    }

    fn emit_assignment_int(&mut self, ir_srcid: &String, xl_dstvar: &String) {
        let ir_dstid = Self::get_ir_id_by_xl_id(&self.var_decls, xl_dstvar);
        Self::ir(&mut self.ir, 1, format!(
            "store i64 {}, i64* {}, align 8"
        , ir_srcid, ir_dstid).as_str());
    }

    fn emit_assignment_str(&mut self, ir_srcid: &String, xl_dstvar: &String) {
        let ir_dstid = Self::get_ir_id_by_xl_id(&self.var_decls, xl_dstvar);
        Self::ir(&mut self.ir, 1, format!(
            "store i8* {}, i8** {}, align 8"
        , ir_srcid, ir_dstid).as_str());
    }

    fn emit_load_booltmpid(&mut self, ir_id: &String) -> String {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir, 1, format!(
            "{} = load i8, i8* {}, align 1"
        , ir_tmpid, ir_id).as_str());
        ir_tmpid
    }

    fn emit_load_inttmpid(&mut self, ir_id: &String) -> String {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir, 1, format!(
            "{} = load i64, i64* {}, align 8"
        , ir_tmpid, ir_id).as_str());
        ir_tmpid
    }

    fn emit_load_strtmpid(&mut self, ir_id: &String) -> String {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir, 1, format!(
            "{} = load i8*, i8** {}, align 8"
        , ir_tmpid, ir_id).as_str());
        ir_tmpid
    }

    fn build_str_decls(&mut self) {
        for step in self.cdata.steps.iter() {
            if let ast::NodeKind::Expression { ref string, .. } = *step.node.kind {
                let xl_val = string;
                if let Some(ref xl_val) = *xl_val {
                    if Self::find_str_decl_by_xl_val(&self.str_decls, xl_val).is_none() {
                        let ir_id = format!("@{}", Self::get_next_id(&mut self.strlit_id));
                        self.str_decls.push(StrDeclaration::from(xl_val).with_id(ir_id));
                    }}}}
    }

    fn build_var_decls(&mut self) {
        for step in self.cdata.steps.iter() {
            if let ast::NodeKind::Assignment { ref identifier, .. } = *step.node.kind {
                let xl_id = identifier;
                if let Some(ref xl_id) = *xl_id {
                    if Self::find_var_decl_by_xl_id(&self.var_decls, xl_id).is_none() {
                        let ir_id = format!("%{}", xl_id);
                        let ir_type = String::from(match step.kind {
                            cdata::StepKind::Bool => "i8",
                            cdata::StepKind::Int  => "i64",
                            cdata::StepKind::Str  => "i8*",
                            _ => unreachable!(), });
                        self.var_decls.push(VarDeclaration::from(xl_id)
                            .with_id(ir_id).with_type(ir_type));
                    }}}}
    }

    fn push_expression(&mut self, step: &'a cdata::Step<'a>) { match *step.node.kind {
        ast::NodeKind::Expression { ref boolean, ref identifier, ref integer, ref string } => {
            let (xl_boolval, xl_id, xl_intval, xl_strval) = (boolean, identifier, integer, string);
            if let Some(ref xl_val) = *xl_boolval {
                let ir_val = match *xl_val { true  => "1", false => "0" }.to_string();
                self.stack.push(Operand::BoolLit { value: ir_val });
            }
            if let Some(ref xl_id) = *xl_id {
                let ir_id = Self::get_ir_id_by_xl_id(&self.var_decls, xl_id).clone();
                match step.kind {
                    cdata::StepKind::Bool => self.stack.push(Operand::BoolVar { ir_id: ir_id }),
                    cdata::StepKind::Int  => self.stack.push(Operand::IntVar  { ir_id: ir_id }),
                    cdata::StepKind::Str  => self.stack.push(Operand::StrVar  { ir_id: ir_id }),
                    _ => unreachable!(), }
            }
            if let Some(ref xl_val) = *xl_intval {
                let ir_val = xl_val.to_string();
                self.stack.push(Operand::IntLit { value: ir_val });
            }
            if let Some(ref xl_val) = *xl_strval {
                let ir_id = Self::get_ir_id_by_xl_val(&self.str_decls, xl_val).clone();
                self.stack.push(Operand::StrLit { value: ir_id });
            }
        }, _ => unreachable!(),
    }}

}


impl<'a> StrDeclaration<'a> {

    fn from(xl_val: &'a String) -> Self {
        let ir_val = Self::ir_val(xl_val);
        let ir_len = Self::ir_len(&ir_val);
        StrDeclaration {
            ir_id: String::from("N/A"),
            ir_val: ir_val,
            ir_len: ir_len,
            xl_val: xl_val }
    }

    fn with_id(mut self, ir_id: String) -> Self {
        self.ir_id = ir_id;
        self
    }

    fn ir_val(xl_val: &String) -> String {
        xl_val
            .replace("\\n",  "\\0A")
            .replace("\\r",  "\\0D")
            .replace("\\t",  "\\09")
            .replace("\\\"", "\\22")
            .replace("\\\\", "\\5C")
            .add("\\00")
    }

    fn ir_len(ir_val: &String) -> usize {
        ir_val.len() - 2 * ir_val.matches('\\').count()
    }

}


impl<'a> VarDeclaration<'a> {

    fn from(xl_id: &'a String) -> Self {
        VarDeclaration {
            ir_id: String::from("N/A"),
            ir_type: String::from("N/A"),
            xl_id: xl_id }
    }

    fn with_id(mut self, ir_id: String) -> Self {
        self.ir_id = ir_id;
        self
    }

    fn with_type(mut self, ir_type: String) -> Self {
        self.ir_type = ir_type;
        self
    }

}


impl<'a> Emitter<'a> {

    fn new(cdata: &'a cdata::CompilerData<'a>) -> Self {
        Emitter {
            cdata: cdata,
            ir: String::new(),
            str_decls: Vec::new(),
            var_decls: Vec::new(),
            stack: Vec::new(),
            strlit_id: 0,
            tmpvar_id: 0 }
    }

    fn find_str_decl_by_ir_id(
               str_decls: &'a Vec<StrDeclaration<'a>>,
               ir_id: &String)
            -> Option<&'a StrDeclaration<'a>> {
        str_decls.iter().find(|e| e.ir_id.as_str() == ir_id)
    }

    fn find_str_decl_by_xl_val(
               str_decls: &'a Vec<StrDeclaration<'a>>,
               xl_val: &String)
            -> Option<&'a StrDeclaration<'a>> {
        str_decls.iter().find(|e| e.xl_val == xl_val)
    }

    fn find_var_decl_by_xl_id(
               var_decls: &'a Vec<VarDeclaration<'a>>,
               xl_id: &String)
            -> Option<&'a VarDeclaration<'a>> {
        var_decls.iter().find(|e| e.xl_id == xl_id)
    }

    fn get_str_decl_by_ir_id(
               str_decls: &'a Vec<StrDeclaration<'a>>,
               ir_id: &String)
            -> &'a StrDeclaration<'a> {
        let str_decl = Self::find_str_decl_by_ir_id(str_decls, ir_id);
        assert!(str_decl.is_some());
        &str_decl.unwrap()
    }

    fn get_ir_id_by_xl_val(
               str_decls: &'a Vec<StrDeclaration<'a>>,
               xl_val: &String)
            -> &'a String {
        let str_decl = Self::find_str_decl_by_xl_val(str_decls, xl_val);
        assert!(str_decl.is_some());
        &str_decl.unwrap().ir_id
    }

    fn get_ir_id_by_xl_id(
               var_decls: &'a Vec<VarDeclaration<'a>>,
               xl_id: &String)
            -> &'a String {
        let var_decl = Self::find_var_decl_by_xl_id(var_decls, xl_id);
        assert!(var_decl.is_some());
        &var_decl.unwrap().ir_id
    }

    fn get_next_id(counter: &mut usize) -> usize {
        let id = *counter;
        *counter += 1;
        id
    }

    fn get_decorated_str_ir_id(str_decl: &'a StrDeclaration<'a>) -> String {
        format!(
            "getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0)"
        , str_decl.ir_len, str_decl.ir_len, str_decl.ir_id)
    }

    fn ir(ir: &mut String, level: usize, line: &str) {
        ir.push_str(format!("{}{}{}", common::take(level, common::TAB),
            line
        , common::NL).as_str());
    }

}
