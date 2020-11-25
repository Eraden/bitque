
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
      regex: Regex::new("\\b(s|tr|y)(?=\\s*[\\(\\[\\{<])"),
      scope: vec![
        Scope {
            a: 61925255089553408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4345 }),
        ContextReference::Direct(ContextId { index: 4339 }),
        ContextReference::Direct(ContextId { index: 4340 }),
        ContextReference::Direct(ContextId { index: 4342 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(s|tr|y)[^\\w\\s\\)\\]\\}\\>])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4345 }),
        ContextReference::Direct(ContextId { index: 4339 }),
        ContextReference::Direct(ContextId { index: 4343 }),
    ]),
      with_prototype: None
    }),
]
} }