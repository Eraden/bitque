
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
      regex: Regex::new("(\\$)(ENV)?(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514781,
            b: 24769797950537728,
        },
    ]),(2, vec![
        Scope {
            a: 52636628119322712,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521962160310,
            b: 24769797950537728,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6195 }),
    ]),
      with_prototype: None
    }),
]
} }