
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
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288689474011277,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(reference|inline|less|css|once|multiple|optional)(\\))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445449949675661,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629318582454,
            b: 39687971716202496,
        },
    ]),(2, vec![
        Scope {
            a: 59955136416251904,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582443,
            b: 39687971716202496,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9123 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9157 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8890 })),
]
} }