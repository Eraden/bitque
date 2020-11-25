
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Fortran (Fixed Form)".to_string(),
  file_extensions: vec!["f".to_string(),"F".to_string(),"f77".to_string(),"F77".to_string(),"for".to_string(),"FOR".to_string(),"fpp".to_string(),"FPP".to_string()],
  scope: Scope { a: 844884491632640, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("program".to_string(), ContextId { index: 7273 });
    v.insert("include_angle_brackets".to_string(), ContextId { index: 7269 });
    v.insert("preprocessor_include".to_string(), ContextId { index: 7272 });
    v.insert("singlestring".to_string(), ContextId { index: 7274 });
    v.insert("__main".to_string(), ContextId { index: 7262 });
    v.insert("formatstring".to_string(), ContextId { index: 7268 });
    v.insert("main".to_string(), ContextId { index: 7270 });
    v.insert("doublestring".to_string(), ContextId { index: 7267 });
    v.insert("common".to_string(), ContextId { index: 7266 });
    v.insert("__start".to_string(), ContextId { index: 7263 });
    v.insert("comment".to_string(), ContextId { index: 7265 });
    v.insert("subroutine_parameter_list".to_string(), ContextId { index: 7275 });
    v.insert("blockdata".to_string(), ContextId { index: 7264 });
    v.insert("preprocessor_errorwarning".to_string(), ContextId { index: 7271 });
    v
  }
} }