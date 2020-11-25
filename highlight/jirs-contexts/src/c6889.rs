
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 6908 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6926 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6910 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\]"),
      scope: vec![
        Scope {
            a: 52636636704866348,
            b: 27866022694354944,
        },
        Scope {
            a: 47288629327233195,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6923 })),
]
} }