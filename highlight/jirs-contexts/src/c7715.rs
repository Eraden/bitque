
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
      regex: Regex::new("(?<=(?:[a-zA-Z0-9_\\x{7f}-\\x{ff}\\]\\)\\\'\\\"]\\|)|\\{%\\sfilter\\s)([a-zA-Z_\\x{7f}-\\x{ff}][a-zA-Z0-9_\\x{7f}-\\x{ff}]*)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444883000492148,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629327560886,
            b: 32651097298436096,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7686 }),
    ]),
      with_prototype: None
    }),
]
} }