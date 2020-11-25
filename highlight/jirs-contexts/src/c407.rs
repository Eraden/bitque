
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 405 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{\\h{8}-(?:\\h{4}-){3}\\h{12}\\}"),
      scope: vec![
        Scope {
            a: 59955136435650571,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(\\d+)(?=\")"),
      scope: vec![
        Scope {
            a: 59955136434012171,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629338046646,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176461530,
            b: 3096224743817216,
        },
        Scope {
            a: 50103314678743051,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }