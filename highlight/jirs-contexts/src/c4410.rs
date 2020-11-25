
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
        a: 46445080558305280,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445080558305280,
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
      regex: Regex::new("@"),
      scope: vec![
        Scope {
            a: 47288629337129022,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4608 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\.?\\s*(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b[ ]*\\.[ ]*)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4557 }),
        ContextReference::Direct(ContextId { index: 4620 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\.?\\s*(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b[ ]*\\.[ ]*)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4558 }),
        ContextReference::Direct(ContextId { index: 4620 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S"),
      scope: vec![
        Scope {
            a: 50103314669371454,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }