
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "F#".to_string(),
  file_extensions: vec!["fs".to_string(),"fsi".to_string(),"fsx".to_string()],
  scope: Scope { a: 844875901698048, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("keywords".to_string(), ContextId { index: 7076 });
    v.insert("modules".to_string(), ContextId { index: 7079 });
    v.insert("record_signature".to_string(), ContextId { index: 7081 });
    v.insert("strp_inlined_body".to_string(), ContextId { index: 7086 });
    v.insert("#anon_anonymous_functions_0".to_string(), ContextId { index: 7024 });
    v.insert("#anon_common_binding_definition_1".to_string(), ContextId { index: 7029 });
    v.insert("#anon_member_declaration_1".to_string(), ContextId { index: 7043 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 7054 });
    v.insert("variables".to_string(), ContextId { index: 7089 });
    v.insert("du_declaration".to_string(), ContextId { index: 7074 });
    v.insert("main".to_string(), ContextId { index: 7077 });
    v.insert("#anon_common_binding_definition_3".to_string(), ContextId { index: 7031 });
    v.insert("#anon_generic_declaration_1".to_string(), ContextId { index: 7040 });
    v.insert("#anon_record_signature_0".to_string(), ContextId { index: 7049 });
    v.insert("attributes".to_string(), ContextId { index: 7064 });
    v.insert("#anon_definition_0".to_string(), ContextId { index: 7036 });
    v.insert("compiler_directives".to_string(), ContextId { index: 7070 });
    v.insert("cexprs".to_string(), ContextId { index: 7065 });
    v.insert("text".to_string(), ContextId { index: 7087 });
    v.insert("constants".to_string(), ContextId { index: 7071 });
    v.insert("#anon_modules_1".to_string(), ContextId { index: 7045 });
    v.insert("#anon_records_0".to_string(), ContextId { index: 7050 });
    v.insert("#anon_common_binding_definition_5".to_string(), ContextId { index: 7033 });
    v.insert("#anon_record_declaration_0".to_string(), ContextId { index: 7047 });
    v.insert("#anon_records_2".to_string(), ContextId { index: 7052 });
    v.insert("#anon_strp_inlined_body_0".to_string(), ContextId { index: 7058 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 7055 });
    v.insert("#anon_modules_2".to_string(), ContextId { index: 7046 });
    v.insert("#anon_common_binding_definition_2".to_string(), ContextId { index: 7030 });
    v.insert("common_binding_definition".to_string(), ContextId { index: 7068 });
    v.insert("tuple_signature".to_string(), ContextId { index: 7088 });
    v.insert("generic_declaration".to_string(), ContextId { index: 7075 });
    v.insert("member_declaration".to_string(), ContextId { index: 7078 });
    v.insert("#anon_generic_declaration_0".to_string(), ContextId { index: 7039 });
    v.insert("#anon_common_declaration_1".to_string(), ContextId { index: 7035 });
    v.insert("#anon_modules_0".to_string(), ContextId { index: 7044 });
    v.insert("#anon_common_binding_definition_4".to_string(), ContextId { index: 7032 });
    v.insert("__start".to_string(), ContextId { index: 7061 });
    v.insert("abstract_definition".to_string(), ContextId { index: 7062 });
    v.insert("#anon_records_3".to_string(), ContextId { index: 7053 });
    v.insert("string_formatter".to_string(), ContextId { index: 7083 });
    v.insert("#anon_abstract_definition_0".to_string(), ContextId { index: 7023 });
    v.insert("__main".to_string(), ContextId { index: 7060 });
    v.insert("records".to_string(), ContextId { index: 7082 });
    v.insert("#anon_strings_2".to_string(), ContextId { index: 7056 });
    v.insert("#anon_attributes_0".to_string(), ContextId { index: 7026 });
    v.insert("strp_inlined".to_string(), ContextId { index: 7085 });
    v.insert("strings".to_string(), ContextId { index: 7084 });
    v.insert("#anon_record_declaration_1".to_string(), ContextId { index: 7048 });
    v.insert("#anon_definition_1".to_string(), ContextId { index: 7037 });
    v.insert("#anon_records_1".to_string(), ContextId { index: 7051 });
    v.insert("#anon_member_declaration_0".to_string(), ContextId { index: 7042 });
    v.insert("chars".to_string(), ContextId { index: 7066 });
    v.insert("#anon_anonymous_functions_1".to_string(), ContextId { index: 7025 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 7027 });
    v.insert("#anon_generic_declaration_2".to_string(), ContextId { index: 7041 });
    v.insert("record_declaration".to_string(), ContextId { index: 7080 });
    v.insert("#anon_du_declaration_0".to_string(), ContextId { index: 7038 });
    v.insert("#anon_strp_inlined_0".to_string(), ContextId { index: 7057 });
    v.insert("anonymous_functions".to_string(), ContextId { index: 7063 });
    v.insert("#anon_common_binding_definition_0".to_string(), ContextId { index: 7028 });
    v.insert("#anon_tuple_signature_0".to_string(), ContextId { index: 7059 });
    v.insert("common_declaration".to_string(), ContextId { index: 7069 });
    v.insert("double_tick".to_string(), ContextId { index: 7073 });
    v.insert("#anon_common_declaration_0".to_string(), ContextId { index: 7034 });
    v.insert("comments".to_string(), ContextId { index: 7067 });
    v.insert("definition".to_string(), ContextId { index: 7072 });
    v
  }
} }