
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
      regex: Regex::new("((?>ACCEPT|COMMIT|FAIL|F))(:?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636746547244,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172844,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6906 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("MARK(?=:\\))"),
      scope: vec![
        Scope {
            a: 52636636746547244,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6906 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?>MARK|THEN|PRUNE|SKIP|(?=:[^)]))"),
      scope: vec![
        Scope {
            a: 52636636746547244,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6883 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6906 })),
]
} }