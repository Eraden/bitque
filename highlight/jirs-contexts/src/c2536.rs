
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
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("class(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 48414576475832363,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2542 }),
        ContextReference::Direct(ContextId { index: 2537 }),
        ContextReference::Direct(ContextId { index: 2539 }),
        ContextReference::Direct(ContextId { index: 2543 }),
    ]),
      with_prototype: None
    }),
]
} }