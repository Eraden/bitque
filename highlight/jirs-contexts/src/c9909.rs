
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
      regex: Regex::new("(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))([+-])?(readonly)\\s*)?\\s*(\\[)\\s*([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s+(in)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628112179373,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033470976,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46445243847868567,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450199,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628121289974,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9756 }),
    ]),
      with_prototype: None
    }),
]
} }