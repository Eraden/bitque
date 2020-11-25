
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
        a: 46444466403475569,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444466403475569,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7554 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7555 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7542 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\()"),
      scope: vec![
        Scope {
            a: 59392130630615295,
            b: 31806672368304128,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7472 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b"),
      scope: vec![
        Scope {
            a: 59392130632974591,
            b: 31806672368304128,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7558 }),
    ]),
      with_prototype: None
    }),
]
} }