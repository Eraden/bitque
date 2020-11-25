
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Dart".to_string(),
  file_extensions: vec!["dart".to_string()],
  scope: Scope { a: 844837246992384, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_regexp_1".to_string(), ContextId { index: 6705 });
    v.insert("#anon_decl-enum_0".to_string(), ContextId { index: 6692 });
    v.insert("__start".to_string(), ContextId { index: 6716 });
    v.insert("#anon_strings_6".to_string(), ContextId { index: 6713 });
    v.insert("annotations".to_string(), ContextId { index: 6717 });
    v.insert("decl-typedef".to_string(), ContextId { index: 6725 });
    v.insert("#anon_decl-function_3".to_string(), ContextId { index: 6700 });
    v.insert("comments-inline".to_string(), ContextId { index: 6719 });
    v.insert("#anon_decl-function_2".to_string(), ContextId { index: 6699 });
    v.insert("#anon_decl-function-parameter_0".to_string(), ContextId { index: 6693 });
    v.insert("#anon_strings_4".to_string(), ContextId { index: 6711 });
    v.insert("decl-class".to_string(), ContextId { index: 6721 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 6708 });
    v.insert("#anon_decl-function-parameter_2".to_string(), ContextId { index: 6695 });
    v.insert("decl-enum".to_string(), ContextId { index: 6722 });
    v.insert("strings".to_string(), ContextId { index: 6730 });
    v.insert("main".to_string(), ContextId { index: 6727 });
    v.insert("string-interp".to_string(), ContextId { index: 6729 });
    v.insert("#anon_decl-function-parameter_3".to_string(), ContextId { index: 6696 });
    v.insert("__main".to_string(), ContextId { index: 6715 });
    v.insert("#anon_strings_3".to_string(), ContextId { index: 6710 });
    v.insert("#anon_strings_2".to_string(), ContextId { index: 6709 });
    v.insert("#anon_string-interp_0".to_string(), ContextId { index: 6706 });
    v.insert("#anon_strings_7".to_string(), ContextId { index: 6714 });
    v.insert("#anon_decl-function_0".to_string(), ContextId { index: 6697 });
    v.insert("#anon_comments-inline_0".to_string(), ContextId { index: 6688 });
    v.insert("#anon_decl-typedef_1".to_string(), ContextId { index: 6702 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6703 });
    v.insert("#anon_comments-inline_2".to_string(), ContextId { index: 6690 });
    v.insert("#anon_strings_5".to_string(), ContextId { index: 6712 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 6707 });
    v.insert("comments".to_string(), ContextId { index: 6718 });
    v.insert("#anon_regexp_0".to_string(), ContextId { index: 6704 });
    v.insert("decl-function-parameter".to_string(), ContextId { index: 6724 });
    v.insert("keywords".to_string(), ContextId { index: 6726 });
    v.insert("regexp".to_string(), ContextId { index: 6728 });
    v.insert("constants-and-special-vars".to_string(), ContextId { index: 6720 });
    v.insert("#anon_decl-typedef_0".to_string(), ContextId { index: 6701 });
    v.insert("#anon_decl-function-parameter_1".to_string(), ContextId { index: 6694 });
    v.insert("#anon_decl-class_0".to_string(), ContextId { index: 6691 });
    v.insert("decl-function".to_string(), ContextId { index: 6723 });
    v.insert("#anon_decl-function_1".to_string(), ContextId { index: 6698 });
    v.insert("#anon_comments-inline_1".to_string(), ContextId { index: 6689 });
    v
  }
} }