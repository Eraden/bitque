
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 2712 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2614 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("default(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636742287403,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2446 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2569 }),
        ContextReference::Direct(ContextId { index: 2627 }),
        ContextReference::Direct(ContextId { index: 2573 }),
        ContextReference::Direct(ContextId { index: 2626 }),
        ContextReference::Direct(ContextId { index: 2572 }),
    ]),
      with_prototype: None
    }),
]
} }