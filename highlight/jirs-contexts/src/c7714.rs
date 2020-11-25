
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
      regex: Regex::new("(?<=(?:[a-zA-Z0-9_\\x{7f}-\\x{ff}\\]\\)\\\'\\\"]\\|)|\\{%\\sfilter\\s)(batch|convert_encoding|date|date_modify|default|e(?:scape)?|format|join|merge|number_format|replace|round|slice|split|trim)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255093157888,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629327560886,
            b: 32651097298436096,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7687 }),
    ]),
      with_prototype: None
    }),
]
} }