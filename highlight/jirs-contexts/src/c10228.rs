
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
        a: 282153581543424,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 282153581543424,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 10238 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^((@)(?:revoked|cert-authority))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445080564596736,
            b: 0,
        },
        Scope {
            a: 49259830331703296,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629337129118,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10224 }),
    ]),
      with_prototype: None
    }),
]
} }