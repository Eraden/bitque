
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Rego".to_string(),
  file_extensions: vec!["rego".to_string()],
  scope: Scope { a: 845013340651520, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 8818 });
    v.insert("__start".to_string(), ContextId { index: 8819 });
    v.insert("comment".to_string(), ContextId { index: 8821 });
    v.insert("operator".to_string(), ContextId { index: 8827 });
    v.insert("#anon_head_0".to_string(), ContextId { index: 8816 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 8817 });
    v.insert("head".to_string(), ContextId { index: 8823 });
    v.insert("main".to_string(), ContextId { index: 8825 });
    v.insert("variable".to_string(), ContextId { index: 8830 });
    v.insert("keyword".to_string(), ContextId { index: 8824 });
    v.insert("call".to_string(), ContextId { index: 8820 });
    v.insert("number".to_string(), ContextId { index: 8826 });
    v.insert("constant".to_string(), ContextId { index: 8822 });
    v.insert("string".to_string(), ContextId { index: 8828 });
    v.insert("term".to_string(), ContextId { index: 8829 });
    v
  }
} }