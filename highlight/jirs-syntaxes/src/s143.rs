
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Puppet".to_string(),
  file_extensions: vec!["pp".to_string(),"epp".to_string()],
  scope: Scope { a: 845000455749632, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_2".to_string(), ContextId { index: 8694 });
    v.insert("nested_braces_interpolated".to_string(), ContextId { index: 8714 });
    v.insert("nested_brackets".to_string(), ContextId { index: 8715 });
    v.insert("strings".to_string(), ContextId { index: 8721 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 8695 });
    v.insert("line_comment".to_string(), ContextId { index: 8711 });
    v.insert("double-quoted-string".to_string(), ContextId { index: 8709 });
    v.insert("#anon_nested_braces_interpolated_0".to_string(), ContextId { index: 8699 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8693 });
    v.insert("#anon_parameter-default-types_0".to_string(), ContextId { index: 8704 });
    v.insert("nested_brackets_interpolated".to_string(), ContextId { index: 8716 });
    v.insert("nested_parens".to_string(), ContextId { index: 8717 });
    v.insert("single-quoted-string".to_string(), ContextId { index: 8720 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 8697 });
    v.insert("#anon_double-quoted-string_0".to_string(), ContextId { index: 8691 });
    v.insert("#anon_nested_parens_interpolated_0".to_string(), ContextId { index: 8703 });
    v.insert("escaped_char".to_string(), ContextId { index: 8710 });
    v.insert("__main".to_string(), ContextId { index: 8706 });
    v.insert("nested_parens_interpolated".to_string(), ContextId { index: 8718 });
    v.insert("parameter-default-types".to_string(), ContextId { index: 8719 });
    v.insert("variable".to_string(), ContextId { index: 8722 });
    v.insert("#anon_nested_brackets_interpolated_0".to_string(), ContextId { index: 8701 });
    v.insert("#anon_nested_braces_0".to_string(), ContextId { index: 8698 });
    v.insert("#anon_nested_parens_0".to_string(), ContextId { index: 8702 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8692 });
    v.insert("#anon_nested_brackets_0".to_string(), ContextId { index: 8700 });
    v.insert("constants".to_string(), ContextId { index: 8708 });
    v.insert("main".to_string(), ContextId { index: 8712 });
    v.insert("nested_braces".to_string(), ContextId { index: 8713 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 8696 });
    v.insert("#anon_single-quoted-string_0".to_string(), ContextId { index: 8705 });
    v.insert("__start".to_string(), ContextId { index: 8707 });
    v
  }
} }