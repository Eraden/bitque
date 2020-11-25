
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
      regex: Regex::new("\\s*(\\b(?:drop|edit|exec|fixup|pick|reword|squash|[defprsx])\\b)\\s+(\\b\\h{7,}\\b)\\s+(.+?)\\s*(?m:$)"),
      scope: vec![
        Scope {
            a: 46446188657180695,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100120599,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136432635925,
            b: 6473924464345088,
        },
    ]),(3, vec![
        Scope {
            a: 55451949139558421,
            b: 6473924464345088,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }