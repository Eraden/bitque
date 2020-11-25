
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
      regex: Regex::new("(?x)\\b(\n    (?i:em|ex|ch|rem)\n  | (?i:vw|vh|vmin|vmax)\n  | (?i:cm|mm|q|in|pt|pc|px|fr)\n  | (?i:deg|grad|rad|turn)\n  | (?i:s|ms)\n  | (?i:Hz|kHz)\n  | (?i:dpi|dpcm|dppx)\n)\\b"),
      scope: vec![
        Scope {
            a: 52636787033309198,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9099 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9131 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9128 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9098 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9148 })),
]
} }