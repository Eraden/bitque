
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
        a: 49259087310291133,
        b: 20829148276588544,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 49259087310291133,
        b: 20829148276588544,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5429 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5371 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[+\\-?]?=|\\s)|(?m:$)|(?=[;&`]|[\\s\\t\\n|&;()<>])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5373 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5444 })),
]
} }