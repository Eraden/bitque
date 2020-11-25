
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
      regex: Regex::new("(?<!\\+\\+|--)(?<=[:=(,\\[?+!>]|^await|[^\\._$\\p{L}\\p{N}]await|^return|[^\\._$\\p{L}\\p{N}]return|^yield|[^\\._$\\p{L}\\p{N}]yield|^throw|[^\\._$\\p{L}\\p{N}]throw|^in|[^\\._$\\p{L}\\p{N}]in|^of|[^\\._$\\p{L}\\p{N}]of|^typeof|[^\\._$\\p{L}\\p{N}]typeof|&&|\\|\\||\\*)\\s*(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325660310,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9394 }),
    ]),
      with_prototype: None
    }),
]
} }