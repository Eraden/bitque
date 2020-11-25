
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Verilog".to_string(),
  file_extensions: vec!["v".to_string(),"V".to_string()],
  scope: Scope { a: 845077765160960, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 10008 });
    v.insert("all-types".to_string(), ContextId { index: 10006 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 10003 });
    v.insert("__main".to_string(), ContextId { index: 10004 });
    v.insert("strings".to_string(), ContextId { index: 10011 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 10001 });
    v.insert("__start".to_string(), ContextId { index: 10005 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 10002 });
    v.insert("storage-modifier-verilog".to_string(), ContextId { index: 10009 });
    v.insert("comments".to_string(), ContextId { index: 10007 });
    v.insert("storage-type-verilog".to_string(), ContextId { index: 10010 });
    v
  }
} }