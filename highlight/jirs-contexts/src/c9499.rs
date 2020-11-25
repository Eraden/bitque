
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
        a: 46445192233222144,
        b: 0,
    },
    Scope {
        a: 46444878704214440,
        b: 42221246506598400,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445192233222144,
        b: 0,
    },
    Scope {
        a: 46444878704214440,
        b: 42221246506598400,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?==>|\\{|(^\\s*(export|function|class|interface|let|var|const|import|enum|namespace|module|type|abstract|declare)\\s+))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9566 })),
]
} }