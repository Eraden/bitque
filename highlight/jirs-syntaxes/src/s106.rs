
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML (EEx)".to_string(),
  file_extensions: vec!["html.eex".to_string(),"html.leex".to_string()],
  scope: Scope { a: 281496458035200, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_eex_tags_3".to_string(), ContextId { index: 6875 });
    v.insert("__main".to_string(), ContextId { index: 6878 });
    v.insert("#anon_eex_tags_0".to_string(), ContextId { index: 6872 });
    v.insert("main".to_string(), ContextId { index: 6881 });
    v.insert("#anon_eex_tags_4".to_string(), ContextId { index: 6876 });
    v.insert("eex_tags".to_string(), ContextId { index: 6880 });
    v.insert("#anon_eex_tags_2".to_string(), ContextId { index: 6874 });
    v.insert("#anon_eex_tags_1".to_string(), ContextId { index: 6873 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6877 });
    v.insert("__start".to_string(), ContextId { index: 6879 });
    v
  }
} }