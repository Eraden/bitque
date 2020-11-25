
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
      regex: Regex::new("(?=(?:[0-9]+(?:<|>>|>)|>>|\\^\\^|[<>^])\\&)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7201 }),
        ContextReference::Direct(ContextId { index: 7202 }),
        ContextReference::Direct(ContextId { index: 7203 }),
        ContextReference::Direct(ContextId { index: 7244 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[0-9]+(?:<|>>|>)|\\&(?:>>|>)|>>|\\^\\^|[<>^])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7204 }),
        ContextReference::Direct(ContextId { index: 7205 }),
        ContextReference::Direct(ContextId { index: 7206 }),
        ContextReference::Direct(ContextId { index: 7249 }),
    ]),
      with_prototype: None
    }),
]
} }