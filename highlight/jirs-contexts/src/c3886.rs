
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46445565958553657,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445565958553657,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 47288629322318029,
            b: 16044073672507392,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:[\\p{L}_:][\\p{L}\\p{N}_]*)+"),
      scope: vec![
        Scope {
            a: 61925255157908562,
            b: 16044073672507392,
        },
    ],
      captures: Some(vec![]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }