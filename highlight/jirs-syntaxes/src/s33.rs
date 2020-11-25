
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Java Server Page (JSP)".to_string(),
  file_extensions: vec!["jsp".to_string()],
  scope: Scope { a: 281496454103040, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 2208 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 2213 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 2215 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 2214 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 2212 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 2210 });
    v.insert("__start".to_string(), ContextId { index: 2217 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 2207 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 2209 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 2211 });
    v.insert("main".to_string(), ContextId { index: 2218 });
    v.insert("__main".to_string(), ContextId { index: 2216 });
    v
  }
} }