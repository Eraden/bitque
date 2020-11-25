
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\U[0-9A-Fa-f]{8})|(\\\\u[0-9A-Fa-f]{4})|(\\\\N\\{[a-zA-Z ]+\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200847315757,
            b: 363666181011275776,
        },
    ]),(2, vec![
        Scope {
            a: 59955200847315757,
            b: 363947655987986432,
        },
    ]),(3, vec![
        Scope {
            a: 59955200847315757,
            b: 59673206163767296,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }