
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
        a: 46445780657963169,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445780657963169,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 10274 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10296 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+(?=[\\s,\"])"),
      scope: vec![
        Scope {
            a: 59955089172922368,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 45317471250415616,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10257 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S"),
      scope: vec![
        Scope {
            a: 55451949107052544,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10258 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10294 })),
]
} }