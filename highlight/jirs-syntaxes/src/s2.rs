
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML (ASP)".to_string(),
  file_extensions: vec!["asp".to_string()],
  scope: Scope { a: 281496451809280, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_asp_directive_0".to_string(), ContextId { index: 81 });
    v.insert("#anon_asp_punctuation_begin_0".to_string(), ContextId { index: 82 });
    v.insert("#anon_html_1".to_string(), ContextId { index: 84 });
    v.insert("begin_embedded_asp".to_string(), ContextId { index: 89 });
    v.insert("close_embedded_asp".to_string(), ContextId { index: 91 });
    v.insert("#anon_html_0".to_string(), ContextId { index: 83 });
    v.insert("__main".to_string(), ContextId { index: 85 });
    v.insert("asp_punctuation_begin".to_string(), ContextId { index: 88 });
    v.insert("begin_embedded_asp_expression".to_string(), ContextId { index: 90 });
    v.insert("html".to_string(), ContextId { index: 92 });
    v.insert("asp_directive".to_string(), ContextId { index: 87 });
    v.insert("main".to_string(), ContextId { index: 93 });
    v.insert("__start".to_string(), ContextId { index: 86 });
    v
  }
} }