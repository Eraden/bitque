
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\{"),
      scope: vec![
        Scope {
            a: 52636787027149709,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6144 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\@"),
      scope: vec![
        Scope {
            a: 52636787027149709,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6145 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(\\#\\s*cmakedefine)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466403475468,
            b: 0,
        },
        Scope {
            a: 52636636717450923,
            b: 3377699720527872,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6146 }),
    ]),
      with_prototype: None
    }),
]
} }