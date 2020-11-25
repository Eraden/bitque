
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
      regex: Regex::new("(\\\\)(.*)(?m:$)\\n?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620736970936,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 50103314739232830,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4462 }),
    ]),
      with_prototype: None
    }),
]
} }