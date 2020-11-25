
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
        index: 3111,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("function(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 48414576474128431,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3096 }),
        ContextReference::Direct(ContextId { index: 3082 }),
        ContextReference::Direct(ContextId { index: 3101 }),
        ContextReference::Direct(ContextId { index: 3097 }),
    ]),
      with_prototype: None
    }),
]
} }