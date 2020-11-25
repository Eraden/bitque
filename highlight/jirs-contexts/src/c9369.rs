
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
      regex: Regex::new("(\\b(resource|provider|variable|output|locals|module|data|terraform)\\b|(\\b(?!null|false|true)[\\p{L}][\\p{L}\\p{N}_-]*\\b))(?=[\\s\\\"\\-[:word:]]*(\\{))"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 48414576472293376,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450197,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9348 }),
    ]),
      with_prototype: None
    }),
]
} }