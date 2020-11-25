
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
      regex: Regex::new("!"),
      scope: vec![
        Scope {
            a: 52636628114800649,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 208 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 193 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 219 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 201 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?!(?:\\+|-|\\*|/|%%|~)|(?:\\||<<|>>|&|\\^)|[=\")])\\S)+"),
      scope: vec![
        Scope {
            a: 49259087310290953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }