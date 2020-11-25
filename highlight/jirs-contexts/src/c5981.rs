
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
      regex: Regex::new("(?x)\n(\\[[^\\]]*?\\])?  # might start with an attribute list (indeed, that is its purpose)\n(?<!\\\\)         # must not be preceded by escape\n(\\#\\#)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925246598250580,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956545,
            b: 56014302054973440,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5931 }),
    ]),
      with_prototype: None
    }),
]
} }