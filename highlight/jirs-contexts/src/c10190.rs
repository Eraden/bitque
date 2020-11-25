
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
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 55451420828565504,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 1407374883553280,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10121 }),
        ContextReference::Direct(ContextId { index: 1033 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10122 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'"),
      scope: vec![
        Scope {
            a: 55451420831973376,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 1407374883553280,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10123 }),
        ContextReference::Direct(ContextId { index: 1033 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10124 }),
    )
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10142 })),
]
} }