
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
      regex: Regex::new("(((?:[a-zA-Z._][a-zA-Z0-9._]*|`[^`]+`))\\s*)((<<?-|=)\\s*)(?=function\\s*\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444131380363328,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615104,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46444131370663936,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130688,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4653 }),
    ]),
      with_prototype: None
    }),
]
} }