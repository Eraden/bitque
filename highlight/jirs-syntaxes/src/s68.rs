
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Ruby Haml".to_string(),
  file_extensions: vec!["haml".to_string(),"sass".to_string()],
  scope: Scope { a: 281767034486784, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("rubyline".to_string(), ContextId { index: 4719 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4709 });
    v.insert("__start".to_string(), ContextId { index: 4716 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 4713 });
    v.insert("__main".to_string(), ContextId { index: 4715 });
    v.insert("#anon_rubyline_0".to_string(), ContextId { index: 4714 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 4712 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 4711 });
    v.insert("continuation".to_string(), ContextId { index: 4717 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 4710 });
    v.insert("main".to_string(), ContextId { index: 4718 });
    v
  }
} }