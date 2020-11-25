
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46452798757601280,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46452798757601280,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7270 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:((?i:end))((?m:$)|\\s*(?i:program)(\\s+\\p{Alpha}\\w*)?))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787159662592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }