
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "reStructuredText".to_string(),
  file_extensions: vec!["rst".to_string(),"rest".to_string()],
  scope: Scope { a: 281775624421376, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 4781 });
    v.insert("__start".to_string(), ContextId { index: 4782 });
    v.insert("main".to_string(), ContextId { index: 4784 });
    v.insert("inline".to_string(), ContextId { index: 4783 });
    v.insert("#anon_inline_2".to_string(), ContextId { index: 4777 });
    v.insert("#anon_inline_3".to_string(), ContextId { index: 4778 });
    v.insert("#anon_inline_0".to_string(), ContextId { index: 4775 });
    v.insert("#anon_inline_1".to_string(), ContextId { index: 4776 });
    v.insert("#anon_inline_4".to_string(), ContextId { index: 4779 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4780 });
    v
  }
} }