
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
  prototype: Some(
    ContextId {
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n  ^(?=\n    (\n       [^<\\x{2190}=\\{\\}/\"]\n      |<[^\\-]\n      |/[^/*]\n      |\"([^\"\\\\]|\\\\\\.)*\"\n      |/\\*([^*]|\\*(?!/))*\\*/\n    )+\n    (?:[[\\p{L}]0-9\\s\\(\\)\\[\\]\\{\\}\']|_)(<-|\\x{2190}|=)[[\\p{L}]0-9\\s\\(\\)\\[\\]\\{\\}\']\n  )"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5145 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5269 })),
]
} }