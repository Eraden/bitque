
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "PHP".to_string(),
  file_extensions: vec!["php".to_string(),"php3".to_string(),"php4".to_string(),"php5".to_string(),"php7".to_string(),"phps".to_string(),"phpt".to_string(),"phtml".to_string()],
  scope: Scope { a: 16607272734031872, b: 0 },
  first_line_match: Some("^(#!.*[^-]php[0-9]?|<\\?php)\\b".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 4159 });
    v.insert("__main".to_string(), ContextId { index: 4161 });
    v.insert("php-end-tag-pop".to_string(), ContextId { index: 4164 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 4160 });
    v.insert("main".to_string(), ContextId { index: 4163 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4158 });
    v.insert("__start".to_string(), ContextId { index: 4162 });
    v
  }
} }