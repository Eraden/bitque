
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
      regex: Regex::new("(?=[\\n\\)#]|;|&(?![|>])|(?:(?:[0-9]+)?(?:[<>]|>>)|\\^\\^?|&)?\\|)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7252 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7231 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7242 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7234 })),
]
} }