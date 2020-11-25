
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
  prototype: Some(
    ContextId {
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(function)\\s+|(?=\\s*[^[\\s\\t\\n|&;()<>]\\d][^[\\s\\t\\n|&;()<>]=]*\\s*\\(\\s*\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576474128458,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5417 }),
        ContextReference::Direct(ContextId { index: 5421 }),
        ContextReference::Direct(ContextId { index: 5420 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcoproc(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636787105529930,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5381 }),
        ContextReference::Direct(ContextId { index: 5398 }),
    ]),
      with_prototype: None
    }),
]
} }