
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)(?:elifn?(?:macro|def|ctx|idni?|id|num|str|token|empty|env)?|else|(?:end(?:if|rep))|exitrep))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314664456447,
            b: 23925746682560512,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }