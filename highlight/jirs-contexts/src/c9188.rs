
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
      regex: Regex::new("\n|(?m:$)|(?=\\{\\s*\\}.*(?m:$))|(?=\\{.*?[:;])|(?=\\{)(?!.+\\}.*(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9203 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\d+(\\.\\d+)?%|from|to"),
      scope: vec![
        Scope {
            a: 59392186616840337,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9227 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("."),
      scope: vec![
        Scope {
            a: 59392186477183121,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }