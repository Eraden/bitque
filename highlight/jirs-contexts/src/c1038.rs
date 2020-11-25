
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
      regex: Regex::new("(?xi)\n    (u\\+)\n    ([0-9a-f?]{1,6}\n    (?:(-)[0-9a-f]{1,6})?)"),
      scope: vec![
        Scope {
            a: 61926827044503552,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925409739964925,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 59956506502496256,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521954754574,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }