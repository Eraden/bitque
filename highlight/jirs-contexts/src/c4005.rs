
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4107 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4129 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\)"),
      scope: vec![
        Scope {
            a: 46445252356734976,
            b: 0,
        },
        Scope {
            a: 47288620745621562,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\|)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445252356734976,
            b: 0,
        },
        Scope {
            a: 61925366772203578,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620747456570,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\$+)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*\\)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087295807488,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }