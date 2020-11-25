
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SQL".to_string(),
  file_extensions: vec!["sql".to_string(),"ddl".to_string(),"dml".to_string()],
  scope: Scope { a: 844721282875392, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("end_identifier".to_string(), "(?=[ \\t]*(?:[^\\w\'\"`. \\t]|$))".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("string_interpolation".to_string(), ContextId { index: 5105 });
    v.insert("comments".to_string(), ContextId { index: 5100 });
    v.insert("#anon_regexps_1".to_string(), ContextId { index: 5093 });
    v.insert("strings".to_string(), ContextId { index: 5106 });
    v.insert("#anon_strings_2".to_string(), ContextId { index: 5096 });
    v.insert("main".to_string(), ContextId { index: 5102 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 5090 });
    v.insert("#anon_regexps_0".to_string(), ContextId { index: 5092 });
    v.insert("identifier_create".to_string(), ContextId { index: 5101 });
    v.insert("#anon_strings_3".to_string(), ContextId { index: 5097 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 5095 });
    v.insert("__start".to_string(), ContextId { index: 5099 });
    v.insert("string_escape".to_string(), ContextId { index: 5104 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 5089 });
    v.insert("#anon_comments_2".to_string(), ContextId { index: 5091 });
    v.insert("__main".to_string(), ContextId { index: 5098 });
    v.insert("regexps".to_string(), ContextId { index: 5103 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 5094 });
    v
  }
} }