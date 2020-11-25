
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "ARM Assembly".to_string(),
  file_extensions: vec!["s".to_string(),"S".to_string()],
  scope: Scope { a: 844790007988224, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 6011 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 6009 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6002 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 6004 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 6007 });
    v.insert("main".to_string(), ContextId { index: 6012 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 6005 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 6003 });
    v.insert("__main".to_string(), ContextId { index: 6010 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6001 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 6006 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 6008 });
    v
  }
} }