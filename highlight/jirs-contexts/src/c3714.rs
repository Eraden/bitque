
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3708 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(?:struct|union|enum\\s+class|enum\\s+struct|enum|class|break|case|catch|continue|default|do|else|for|goto|if|_Pragma|return|switch|throw|try|while)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3793 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3765 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3764 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 657 })),
]
} }