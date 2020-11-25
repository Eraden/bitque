
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
      regex: Regex::new("(?=\\b(type|val|external|class|module|end)\\b)|^\\s*(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?)([a-z][a-zA-Z0-9_]*)\\s*(:)"),
      scope: vec![
        Scope {
            a: 49258876907291673,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629379858484,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122721,
            b: 295267473907777536,
        },
    ]),(3, vec![
        Scope {
            a: 47288620789923892,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([a-z][a-zA-Z0-9\'_]*)\\s*(:)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122721,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 47288620744245300,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3492 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3520 })),
]
} }