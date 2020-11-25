
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Robot Framework syntax highlighting.".to_string(),
  file_extensions: vec!["robot".to_string()],
  scope: Scope { a: 845021930586112, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 8841 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 8837 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8834 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8835 });
    v.insert("__start".to_string(), ContextId { index: 8840 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 8836 });
    v.insert("__main".to_string(), ContextId { index: 8839 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 8838 });
    v
  }
} }