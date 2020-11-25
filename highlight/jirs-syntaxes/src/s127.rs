
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML (Jinja2)".to_string(),
  file_extensions: vec!["htm.j2".to_string(),"html.j2".to_string(),"xhtml.j2".to_string(),"xml.j2".to_string()],
  scope: Scope { a: 281496459345920, b: 0 },
  first_line_match: Some("^{% extends [\"\'][^\"\']+[\"\'] %}".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 7856 });
    v.insert("__start".to_string(), ContextId { index: 7857 });
    v.insert("main".to_string(), ContextId { index: 7858 });
    v
  }
} }