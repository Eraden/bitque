
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
      regex: Regex::new("(<?)mailto:"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 29273397577908224,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6970 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<?)((?i:[-a-z0-9._+]+@[-a-z0-9.]+))(>?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 29273397577908224,
        },
    ]),(2, vec![
        Scope {
            a: 114280588597985948,
            b: 29273397577908224,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 29273397577908224,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }