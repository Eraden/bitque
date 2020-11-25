
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
      regex: Regex::new("(?<!,)(((?==|;|}|((?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(of|in)\\s+)|^\\s*(?m:$)))|((?<=\\S)(?=\\s*(?m:$))))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9663 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9583 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9696 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9653 })),
]
} }