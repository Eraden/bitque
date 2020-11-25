
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
      regex: Regex::new("(?x)\n(<\\?) # opening <? punctuation\n(?:\n  # valid lowercase prolog tag name\n  (xml(?:-[_a-z][-_a-z0-9]*)?)(?=[?<>\\s])\n  |\n  # invalid mixed or uppercase tag name\n  ([xX][mM][lL][^?<>\\s]*)\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122448,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314750373968,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5776 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<\\?)([\\p{L}:_][\\p{L}\\p{N}:_.-]*)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122448,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5777 }),
    ]),
      with_prototype: None
    }),
]
} }