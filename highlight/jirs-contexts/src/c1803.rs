
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
        a: 281565172727808,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281565172727808,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 1740 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("!|(?=\\S)"),
      scope: vec![
        Scope {
            a: 52636628114801078,
            b: 188588324592156672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1804 }),
        ContextReference::Direct(ContextId { index: 1753 }),
    ]),
      with_prototype: None
    }),
]
} }