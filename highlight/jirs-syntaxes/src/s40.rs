
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "LaTeX Log".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 281595238219776, b: 0 },
  first_line_match: Some("This is (pdf|pdfe)?TeXk?, Version ".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 2748 });
    v.insert("__start".to_string(), ContextId { index: 2750 });
    v.insert("main".to_string(), ContextId { index: 2751 });
    v.insert("__main".to_string(), ContextId { index: 2749 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 2747 });
    v
  }
} }