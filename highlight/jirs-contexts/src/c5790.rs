
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5818 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5792 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:CDATA|ENTITY|ENTITIES|IDREFS?|ID|NMTOKENS?|NOTATION)\\b"),
      scope: vec![
        Scope {
            a: 48414576491888720,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#)(?:FIXED|IMPLIED|REQUIRED)\\b"),
      scope: vec![
        Scope {
            a: 48414439052935602,
            b: 22517998136852480,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322317904,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{L}:_][\\p{L}\\p{N}:_.-]*\\b"),
      scope: vec![
        Scope {
            a: 59392186477183056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5794 })),
]
} }