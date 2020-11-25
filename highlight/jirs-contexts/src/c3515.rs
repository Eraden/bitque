
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
      regex: Regex::new("\\b_\\b"),
      scope: vec![
        Scope {
            a: 59955110707593268,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|(?=\\s*\\S)"),
      scope: vec![
        Scope {
            a: 47288620791562292,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(?=(?!=.*?->).*?\\|)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629379596340,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3490 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3517 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3511 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3521 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3514 })),
]
} }