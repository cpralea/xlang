use common;

use super::emitter;
use super::strdecl;
use super::vardecl;


impl<'a> emitter::Emitter<'a> {
    pub fn emit_load_booltmpid(&mut self, ir_id: &String) -> String {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = load i8, i8* {}, align 1", ir_tmpid, ir_id).as_str());
        ir_tmpid
    }

    pub fn emit_load_inttmpid(&mut self, ir_id: &String) -> String {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = load i64, i64* {}, align 8", ir_tmpid, ir_id).as_str());
        ir_tmpid
    }

    pub fn emit_load_strtmpid(&mut self, ir_id: &String) -> String {
        let ir_tmpid = format!("%{}", Self::get_next_id(&mut self.tmpvar_id));
        Self::ir(&mut self.ir,
                 1,
                 format!("{} = load i8*, i8** {}, align 8", ir_tmpid, ir_id).as_str());
        ir_tmpid
    }
}


impl<'a> emitter::Emitter<'a> {
    pub fn find_str_decl_by_ir_id(str_decls: &'a Vec<strdecl::StrDecl<'a>>,
                                  ir_id: &String)
                                  -> Option<&'a strdecl::StrDecl<'a>> {
        str_decls.iter().find(|e| e.ir_id.as_str() == ir_id)
    }

    pub fn find_str_decl_by_xl_val(str_decls: &'a Vec<strdecl::StrDecl<'a>>,
                                   xl_val: &String)
                                   -> Option<&'a strdecl::StrDecl<'a>> {
        str_decls.iter().find(|e| e.xl_val == xl_val)
    }

    pub fn find_var_decl_by_xl_id(var_decls: &'a Vec<vardecl::VarDecl<'a>>,
                                  xl_id: &String)
                                  -> Option<&'a vardecl::VarDecl<'a>> {
        var_decls.iter().find(|e| e.xl_id == xl_id)
    }

    pub fn get_str_decl_by_ir_id(str_decls: &'a Vec<strdecl::StrDecl<'a>>,
                                 ir_id: &String)
                                 -> &'a strdecl::StrDecl<'a> {
        let str_decl = Self::find_str_decl_by_ir_id(str_decls, ir_id);
        assert!(str_decl.is_some());
        &str_decl.unwrap()
    }

    pub fn get_ir_id_by_xl_val(str_decls: &'a Vec<strdecl::StrDecl<'a>>,
                               xl_val: &String)
                               -> &'a String {
        let str_decl = Self::find_str_decl_by_xl_val(str_decls, xl_val);
        assert!(str_decl.is_some());
        &str_decl.unwrap().ir_id
    }

    pub fn get_ir_id_by_xl_id(var_decls: &'a Vec<vardecl::VarDecl<'a>>,
                              xl_id: &String)
                              -> &'a String {
        let var_decl = Self::find_var_decl_by_xl_id(var_decls, xl_id);
        assert!(var_decl.is_some());
        &var_decl.unwrap().ir_id
    }

    pub fn get_next_id(counter: &mut usize) -> usize {
        let id = *counter;
        *counter += 1;
        id
    }

    pub fn get_decorated_str_ir_id(str_decl: &'a strdecl::StrDecl<'a>) -> String {
        format!("getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0)",
                str_decl.ir_len,
                str_decl.ir_len,
                str_decl.ir_id)
    }

    pub fn ir(ir: &mut String, level: usize, line: &str) {
        ir.push_str(format!("{}{}{}", common::take(level, common::TAB), line, common::NL)
                        .as_str());
    }
}
