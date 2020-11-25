
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 2386 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:class|(\\@?)interface)\\b"),
      scope: vec![
        Scope {
            a: 48414576465149952,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324480552,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2307 }),
        ContextReference::Direct(ContextId { index: 2309 }),
        ContextReference::Direct(ContextId { index: 2329 }),
        ContextReference::Direct(ContextId { index: 2311 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("enum\\b"),
      scope: vec![
        Scope {
            a: 48414576465149952,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2321 }),
        ContextReference::Direct(ContextId { index: 2309 }),
        ContextReference::Direct(ContextId { index: 2329 }),
        ContextReference::Direct(ContextId { index: 2311 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2298 })),
]
} }