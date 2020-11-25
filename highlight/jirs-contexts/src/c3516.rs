
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
      regex: Regex::new("(val)\\s+([a-z_][a-zA-Z0-9_\']*)\\s*(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787015942144,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632451124,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 47288620788351028,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3491 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(external)\\s+([a-z_][a-zA-Z0-9_\']*)\\s*(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787015942144,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632451125,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 47288620788351028,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3493 }),
    ]),
      with_prototype: None
    }),
]
} }