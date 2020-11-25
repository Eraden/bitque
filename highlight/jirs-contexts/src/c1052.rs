
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1054 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:nil|true|false)(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\])"),
      scope: vec![
        Scope {
            a: 59955110638190592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1068 }),
    ]),
      with_prototype: None
    }),
]
} }