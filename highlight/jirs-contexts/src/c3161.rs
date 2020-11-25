
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
        a: 49259087295152128,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 49259087295152128,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3172 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3199 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?|\\+|::?)?="),
      scope: vec![
        Scope {
            a: 52636628111130672,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3196 }),
        ContextReference::Direct(ContextId { index: 3174 }),
    ]),
      with_prototype: None
    }),
]
} }