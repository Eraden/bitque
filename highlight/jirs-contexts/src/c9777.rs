
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
      regex: Regex::new("(?<=\\))|(?=[;),}\\]:]|\\|\\||\\&\\&|(?m:$)|((?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))new(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.)))|((?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))function((\\s+[_$\\p{L}][_$\\p{L}\\p{N}]*)|(\\s*[\\(]))))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9952 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9868 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9977 })),
]
} }