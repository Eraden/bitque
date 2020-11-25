
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
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 47288629318582443,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(even|odd)\\b"),
      scope: vec![
        Scope {
            a: 52636787045695502,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([-+]?\\d*(n))\\s*([-+]\\s*\\d+)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 52636787033309198,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089176461530,
            b: 3940649673949184,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[-+]\\s+\\d+n?|[-+]?\\d+\\s+n"),
      scope: vec![
        Scope {
            a: 50103314667667470,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8293 })),
]
} }