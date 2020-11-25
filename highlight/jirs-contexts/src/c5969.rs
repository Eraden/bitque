
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
      regex: Regex::new("(?<!\\()(\\({3})([^\\(].*?)(\\){3})(?!\\))"),
      scope: vec![
        Scope {
            a: 49259087395292534,
            b: 23643898043695104,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200934806902,
            b: 51228806538592256,
        },
    ]),(3, vec![
        Scope {
            a: 59955200934806902,
            b: 48132581794775040,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }