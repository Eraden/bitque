
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
      regex: Regex::new("(?xi:(?=\\b end \\s* (!|;|(?m:$)|&|(?xi:block\\s+data|function|module|program|subroutine|submodule|type|interface|type)\\b) (\\s+[A-Za-z_][A-Za-z_0-9]*)?\\b))"),
      scope: vec![
        Scope {
            a: 638948978817236992,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7287 }),
    ]),
      with_prototype: None
    }),
]
} }