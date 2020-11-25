
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
      regex: Regex::new("(?==>|\\{|(^\\s*(export|function|class|interface|let|var|const|import|enum|namespace|module|type|abstract|declare)\\s+))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9688 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9607 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9565 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9650 })),
]
} }