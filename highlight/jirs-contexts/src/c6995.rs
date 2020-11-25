
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
      regex: Regex::new("^\\n(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6984 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^((?i:Content-Type))(:)\\s*(image/\\w+)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244992,
            b: 29273397577908224,
        },
        Scope {
            a: 52636787019350016,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757876985,
            b: 29273397577908224,
        },
    ]),(3, vec![
        Scope {
            a: 59392130642804840,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6999 }),
        ContextReference::Direct(ContextId { index: 7005 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7001 })),
]
} }