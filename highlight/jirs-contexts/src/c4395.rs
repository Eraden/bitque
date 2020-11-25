
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46444268822921618,
        b: 17451448556060672,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444268822921618,
        b: 17451448556060672,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 4608 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 46444268822921618,
            b: 17451448556060672,
        },
        Scope {
            a: 47288521951478027,
            b: 113153206925656064,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4615 })),
]
} }