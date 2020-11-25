
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
        index: 1792,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s*)([a-zA-Z][\\w-]*)(\\s*(\\=)\\s*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780635287577,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445780653244437,
            b: 7036874417766400,
        },
        Scope {
            a: 49259087310290965,
            b: 7036874417766400,
        },
    ]),(3, vec![
        Scope {
            a: 46445780635287577,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130645,
            b: 7036874417766400,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1768 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1783 })),
]
} }