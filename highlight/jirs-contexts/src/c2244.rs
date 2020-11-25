
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
  prototype: Some(
    ContextId {
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2304 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:((?:[_$]*\\p{Lu}[\\p{Lu}\\p{N}_$]*\\b))|((?:[\\p{L}_$][\\p{L}\\p{N}_$]*)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632974376,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445999679864832,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2383 }),
        ContextReference::Direct(ContextId { index: 2299 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2380 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2298 })),
]
} }