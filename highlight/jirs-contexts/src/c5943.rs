
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
      regex: Regex::new("({)([A-Za-z0-9_][A-Za-z0-9_-]*)(})"),
      scope: vec![
        Scope {
            a: 49259087292006400,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200832962734,
            b: 51228806538592256,
        },
    ]),(2, vec![
        Scope {
            a: 61925246524981332,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955200832962734,
            b: 48132581794775040,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }