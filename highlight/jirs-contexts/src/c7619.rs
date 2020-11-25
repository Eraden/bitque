
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
        index: 7635,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:\n  QUERY|MUTATION|SUBSCRIPTION|FIELD|FRAGMENT_DEFINITION|FRAGMENT_SPREAD|INLINE_FRAGMENT\n  |SCHEMA|SCALAR|OBJECT|FIELD_DEFINITION|ARGUMENT_DEFINITION|INTERFACE|UNION|ENUM|ENUM_VALUE|INPUT_OBJECT|INPUT_FIELD_DEFINITION\n)(?!(?:[_0-9A-Za-z]))\n"),
      scope: vec![
        Scope {
            a: 59955110644678656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7622 })),
]
} }