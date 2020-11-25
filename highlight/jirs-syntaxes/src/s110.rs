
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Elm".to_string(),
  file_extensions: vec!["elm".to_string()],
  scope: Scope { a: 844867311763456, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_module_exports_0".to_string(), ContextId { index: 6952 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6946 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 6947 });
    v.insert("comments".to_string(), ContextId { index: 6956 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 6950 });
    v.insert("__start".to_string(), ContextId { index: 6954 });
    v.insert("__main".to_string(), ContextId { index: 6953 });
    v.insert("infix_op".to_string(), ContextId { index: 6957 });
    v.insert("type_signature".to_string(), ContextId { index: 6961 });
    v.insert("main".to_string(), ContextId { index: 6958 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 6951 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6945 });
    v.insert("#anon_block_comment_0".to_string(), ContextId { index: 6944 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 6948 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 6949 });
    v.insert("module_exports".to_string(), ContextId { index: 6959 });
    v.insert("module_name".to_string(), ContextId { index: 6960 });
    v.insert("block_comment".to_string(), ContextId { index: 6955 });
    v
  }
} }