
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Link".to_string(),
  file_extensions: vec![".git".to_string()],
  scope: Scope { a: 281565172793344, b: 0 },
  first_line_match: Some("^\\s*gitdir\\s*:".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("separator".to_string(), ContextId { index: 1814 });
    v.insert("gitdir".to_string(), ContextId { index: 1809 });
    v.insert("path".to_string(), ContextId { index: 1811 });
    v.insert("path-start".to_string(), ContextId { index: 1813 });
    v.insert("main".to_string(), ContextId { index: 1810 });
    v.insert("__main".to_string(), ContextId { index: 1805 });
    v.insert("__start".to_string(), ContextId { index: 1806 });
    v.insert("path-dir-pattern".to_string(), ContextId { index: 1812 });
    v.insert("expect-path".to_string(), ContextId { index: 1807 });
    v.insert("expect-separator".to_string(), ContextId { index: 1808 });
    v
  }
} }