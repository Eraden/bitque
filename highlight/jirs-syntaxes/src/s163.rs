
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "VimL".to_string(),
  file_extensions: vec!["vim".to_string(),".vimrc".to_string()],
  scope: Scope { a: 845082060128256, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("function_call".to_string(), ContextId { index: 10018 });
    v.insert("string_quoted_double".to_string(), ContextId { index: 10026 });
    v.insert("string_regex".to_string(), ContextId { index: 10028 });
    v.insert("expr".to_string(), ContextId { index: 10016 });
    v.insert("function_definition".to_string(), ContextId { index: 10019 });
    v.insert("support_function".to_string(), ContextId { index: 10029 });
    v.insert("inline_comment".to_string(), ContextId { index: 10021 });
    v.insert("support_variable".to_string(), ContextId { index: 10031 });
    v.insert("number_hex".to_string(), ContextId { index: 10024 });
    v.insert("support_type".to_string(), ContextId { index: 10030 });
    v.insert("#anon_function_definition_0".to_string(), ContextId { index: 10012 });
    v.insert("variable".to_string(), ContextId { index: 10032 });
    v.insert("keyword".to_string(), ContextId { index: 10022 });
    v.insert("string_quoted_single".to_string(), ContextId { index: 10027 });
    v.insert("__main".to_string(), ContextId { index: 10013 });
    v.insert("function".to_string(), ContextId { index: 10017 });
    v.insert("__start".to_string(), ContextId { index: 10014 });
    v.insert("function_params".to_string(), ContextId { index: 10020 });
    v.insert("main".to_string(), ContextId { index: 10023 });
    v.insert("number_int".to_string(), ContextId { index: 10025 });
    v.insert("comment".to_string(), ContextId { index: 10015 });
    v
  }
} }