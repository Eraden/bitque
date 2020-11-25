
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
      regex: Regex::new("(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629349777460,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwith\\b"),
      scope: vec![
        Scope {
            a: 52636787026886708,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\bmutable\\s+)?\\b([a-z_][a-zA-Z0-9_\']*)\\s*(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787023806637,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 844648268431360,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629349777460,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3489 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3514 })),
]
} }