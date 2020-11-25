
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
      regex: Regex::new("\\b(character|complex|double|expression|integer|list|logical|numeric|single|raw)\\s*(?=\\()"),
      scope: vec![
        Scope {
            a: 46444882999640128,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466722816,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4668 }),
    ]),
      with_prototype: None
    }),
]
} }