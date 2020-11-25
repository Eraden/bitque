
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
      regex: Regex::new("(--)(pretty|format)(=)(oneline|short|medium|fuller|full|email|raw)?(?:(t?format)(:))?"),
      scope: vec![],
      captures: Some(vec![(5, vec![
        Scope {
            a: 52638212985126933,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 52636628111131290,
            b: 186617999753478144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1755 }),
    ]),
      with_prototype: None
    }),
]
} }