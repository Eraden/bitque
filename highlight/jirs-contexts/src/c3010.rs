
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
  prototype: Some(
    ContextId {
        index: 3019,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\def)\\s*((\\\\)[A-Za-z@]+)\\s*[^\\{]*?\\s*(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255098269732,
            b: 0,
        },
        Scope {
            a: 48414439036289060,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925255144931364,
            b: 0,
        },
        Scope {
            a: 59392130631729188,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629362032676,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629318582708,
            b: 51228600380162048,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2995 }),
    ]),
      with_prototype: None
    }),
]
} }