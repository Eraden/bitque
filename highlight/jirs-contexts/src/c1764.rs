
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
      regex: Regex::new("(%\\()(trailers)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521948004534,
            b: 186617999753478144,
        },
    ]),(2, vec![
        Scope {
            a: 52636787056116375,
            b: 5910974510923776,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1735 }),
    ]),
      with_prototype: None
    }),
]
} }