
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Kotlin".to_string(),
  file_extensions: vec!["kt".to_string(),"kts".to_string()],
  scope: Scope { a: 844948916142080, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_strings_3".to_string(), ContextId { index: 8053 });
    v.insert("#anon_types_2".to_string(), ContextId { index: 8058 });
    v.insert("typedefs".to_string(), ContextId { index: 8082 });
    v.insert("namespaces".to_string(), ContextId { index: 8078 });
    v.insert("#anon_functions_2".to_string(), ContextId { index: 8034 });
    v.insert("#anon_functions_1".to_string(), ContextId { index: 8033 });
    v.insert("#anon_getters-and-setters_0".to_string(), ContextId { index: 8040 });
    v.insert("#anon_types_3".to_string(), ContextId { index: 8059 });
    v.insert("#anon_variables_0".to_string(), ContextId { index: 8060 });
    v.insert("#anon_functions_5".to_string(), ContextId { index: 8037 });
    v.insert("#anon_parameters_0".to_string(), ContextId { index: 8048 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 8051 });
    v.insert("#anon_functions_0".to_string(), ContextId { index: 8032 });
    v.insert("#anon_functions_4".to_string(), ContextId { index: 8036 });
    v.insert("#anon_getters-and-setters_5".to_string(), ContextId { index: 8045 });
    v.insert("#anon_strings_2".to_string(), ContextId { index: 8052 });
    v.insert("classes".to_string(), ContextId { index: 8068 });
    v.insert("parameters".to_string(), ContextId { index: 8079 });
    v.insert("statements".to_string(), ContextId { index: 8080 });
    v.insert("comments".to_string(), ContextId { index: 8069 });
    v.insert("#anon_functions_6".to_string(), ContextId { index: 8038 });
    v.insert("#anon_functions_3".to_string(), ContextId { index: 8035 });
    v.insert("expressions".to_string(), ContextId { index: 8071 });
    v.insert("keywords".to_string(), ContextId { index: 8076 });
    v.insert("#anon_parameters_1".to_string(), ContextId { index: 8049 });
    v.insert("#anon_variables_5".to_string(), ContextId { index: 8065 });
    v.insert("#anon_getters-and-setters_6".to_string(), ContextId { index: 8046 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 8050 });
    v.insert("main".to_string(), ContextId { index: 8077 });
    v.insert("#anon_getters-and-setters_1".to_string(), ContextId { index: 8041 });
    v.insert("#anon_getters-and-setters_4".to_string(), ContextId { index: 8044 });
    v.insert("getters-and-setters".to_string(), ContextId { index: 8074 });
    v.insert("#anon_classes_0".to_string(), ContextId { index: 8023 });
    v.insert("#anon_variables_1".to_string(), ContextId { index: 8061 });
    v.insert("#anon_types_0".to_string(), ContextId { index: 8056 });
    v.insert("__start".to_string(), ContextId { index: 8067 });
    v.insert("strings".to_string(), ContextId { index: 8081 });
    v.insert("#anon_variables_4".to_string(), ContextId { index: 8064 });
    v.insert("__main".to_string(), ContextId { index: 8066 });
    v.insert("#anon_types_1".to_string(), ContextId { index: 8057 });
    v.insert("#anon_classes_3".to_string(), ContextId { index: 8026 });
    v.insert("#anon_classes_5".to_string(), ContextId { index: 8028 });
    v.insert("#anon_expressions_0".to_string(), ContextId { index: 8031 });
    v.insert("#anon_getters-and-setters_2".to_string(), ContextId { index: 8042 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 8030 });
    v.insert("imports".to_string(), ContextId { index: 8075 });
    v.insert("#anon_classes_2".to_string(), ContextId { index: 8025 });
    v.insert("#anon_getters-and-setters_3".to_string(), ContextId { index: 8043 });
    v.insert("#anon_typedefs_1".to_string(), ContextId { index: 8055 });
    v.insert("generics".to_string(), ContextId { index: 8073 });
    v.insert("#anon_typedefs_0".to_string(), ContextId { index: 8054 });
    v.insert("#anon_variables_2".to_string(), ContextId { index: 8062 });
    v.insert("#anon_variables_3".to_string(), ContextId { index: 8063 });
    v.insert("#anon_classes_4".to_string(), ContextId { index: 8027 });
    v.insert("variables".to_string(), ContextId { index: 8084 });
    v.insert("#anon_generics_0".to_string(), ContextId { index: 8039 });
    v.insert("#anon_classes_1".to_string(), ContextId { index: 8024 });
    v.insert("#anon_namespaces_0".to_string(), ContextId { index: 8047 });
    v.insert("#anon_classes_6".to_string(), ContextId { index: 8029 });
    v.insert("constants".to_string(), ContextId { index: 8070 });
    v.insert("functions".to_string(), ContextId { index: 8072 });
    v.insert("types".to_string(), ContextId { index: 8083 });
    v
  }
} }