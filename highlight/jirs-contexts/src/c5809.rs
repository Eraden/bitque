
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
      regex: Regex::new("(<!)(NOTATION)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 52638213038866512,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5810 }),
        ContextReference::Direct(ContextId { index: 5794 }),
        ContextReference::Direct(ContextId { index: 5795 }),
        ContextReference::Direct(ContextId { index: 5811 }),
    ]),
      with_prototype: None
    }),
]
} }