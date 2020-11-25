
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
        a: 638957951003918336,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 638957951003918336,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7370 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\bend)"),
      scope: vec![
        Scope {
            a: 52636636699887813,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7288 }),
    ]),
      with_prototype: None
    }),
]
} }