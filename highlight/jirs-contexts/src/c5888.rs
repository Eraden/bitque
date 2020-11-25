
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
        a: 55450759390560256,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 55450759390560256,
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
      regex: Regex::new("(/)([gimy]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629385691307,
            b: 23080948090273792,
        },
    ]),(2, vec![
        Scope {
            a: 52636787017908224,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5903 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=.|\\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5889 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 5890 }),
    )
    }),
]
} }