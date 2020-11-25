
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
        a: 46444328940077056,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444328940077056,
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
      regex: Regex::new("(\\})([^[;\\s\\}\\]\\\\]]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444328940077056,
            b: 0,
        },
        Scope {
            a: 47288521951477931,
            b: 21955048183431168,
        },
    ]),(2, vec![
        Scope {
            a: 50103314658754560,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5737 })),
]
} }