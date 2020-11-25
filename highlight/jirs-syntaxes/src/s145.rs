
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "QML".to_string(),
  file_extensions: vec!["qml".to_string(),"qmlproject".to_string()],
  scope: Scope { a: 845009045684224, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 8814 });
    v.insert("__main".to_string(), ContextId { index: 8813 });
    v.insert("main".to_string(), ContextId { index: 8815 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8810 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8811 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 8812 });
    v
  }
} }