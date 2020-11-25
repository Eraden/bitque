
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 2305 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[_$]*\\p{Ll}[\\p{Ll}\\p{N}_$]*\\b)"),
      scope: vec![
        Scope {
            a: 46445252355555328,
            b: 0,
        },
        Scope {
            a: 61925375391039528,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2275 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2298 })),
]
} }