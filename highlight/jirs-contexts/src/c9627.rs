
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
      regex: Regex::new("(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?(?:(\\bdeclare)\\s+)?\\b(namespace|module)\\s+(?=[_$\\p{L}\"\'`]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636728328342,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576487039126,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9468 }),
    ]),
      with_prototype: None
    }),
]
} }