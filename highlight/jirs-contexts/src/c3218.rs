
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
      regex: Regex::new("(?x)\n    [ \\t]*_{4,}    # if there are more than 3 its not applicable to be bold or italic\n|   [ \\t]+__+      # whitespace followed by 2 or more is also not applicable\n|   ^__            # emphasis can\'t be closed at the start of the line"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\*)?(__\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629354365099,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3337 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*(?=[^\\s*])(?=[^*_]*__\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3340 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3312 })),
]
} }