
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
      regex: Regex::new("(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(readonly)\\s*)?\\s*(\\[)\\s*([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445243847868566,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258876848439296,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9457 }),
    ]),
      with_prototype: None
    }),
]
} }