
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Haskell".to_string(),
  file_extensions: vec!["hs".to_string()],
  scope: Scope { a: 844575253987328, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_pragma_0".to_string(), ContextId { index: 2167 });
    v.insert("main".to_string(), ContextId { index: 2173 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 2164 });
    v.insert("module_name".to_string(), ContextId { index: 2175 });
    v.insert("__start".to_string(), ContextId { index: 2169 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 2161 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 2159 });
    v.insert("module_exports".to_string(), ContextId { index: 2174 });
    v.insert("pragma".to_string(), ContextId { index: 2176 });
    v.insert("infix_op".to_string(), ContextId { index: 2172 });
    v.insert("type_signature".to_string(), ContextId { index: 2177 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 2165 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 2163 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 2156 });
    v.insert("#anon_block_comment_0".to_string(), ContextId { index: 2154 });
    v.insert("__main".to_string(), ContextId { index: 2168 });
    v.insert("#anon_module_exports_0".to_string(), ContextId { index: 2166 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 2160 });
    v.insert("block_comment".to_string(), ContextId { index: 2170 });
    v.insert("comments".to_string(), ContextId { index: 2171 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 2157 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 2162 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 2158 });
    v.insert("#anon_block_comment_1".to_string(), ContextId { index: 2155 });
    v
  }
} }