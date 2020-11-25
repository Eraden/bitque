
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML (Erlang)".to_string(),
  file_extensions: vec!["yaws".to_string()],
  scope: Scope { a: 281496452857856, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("break_char".to_string(), "[\\t\\n\\f /<>]".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_tag-erlang-body_1".to_string(), ContextId { index: 1678 });
    v.insert("tag-erlang-attributes".to_string(), ContextId { index: 1684 });
    v.insert("#anon_tag-erlang-body_2".to_string(), ContextId { index: 1679 });
    v.insert("tag-erlang".to_string(), ContextId { index: 1683 });
    v.insert("main".to_string(), ContextId { index: 1682 });
    v.insert("tag-erlang-body".to_string(), ContextId { index: 1685 });
    v.insert("__start".to_string(), ContextId { index: 1681 });
    v.insert("#anon_tag-erlang-body_0".to_string(), ContextId { index: 1677 });
    v.insert("__main".to_string(), ContextId { index: 1680 });
    v
  }
} }