
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
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1006 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 998 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 992 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1042 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1022 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 990 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[-/*+]"),
      scope: vec![
        Scope {
            a: 52636628099661824,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288629318582454,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1011 }),
    ]),
      with_prototype: None
    }),
]
} }