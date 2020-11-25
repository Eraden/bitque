
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
      regex: Regex::new("(<?)(https?:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 29273397577908224,
        },
    ]),(2, vec![
        Scope {
            a: 114280588597985825,
            b: 29273397577908224,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6978 }),
    ]),
      with_prototype: None
    }),
]
} }