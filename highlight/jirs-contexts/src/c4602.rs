
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
      regex: Regex::new("\\b(lambda)(?=\\s|:|(?m:$))"),
      scope: vec![
        Scope {
            a: 48414576474129137,
            b: 17451448556060672,
        },
        Scope {
            a: 52638212953277169,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4603 }),
        ContextReference::Direct(ContextId { index: 4545 }),
    ]),
      with_prototype: None
    }),
]
} }