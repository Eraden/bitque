
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
      regex: Regex::new("(?<=(?:[a-zA-Z0-9_\\x{7f}-\\x{ff}\\]\\)\\\'\\\"]\\|)|\\{%\\sfilter\\s)(abs|capitalize|e(?:scape)?|first|join|(?:json|url)_encode|keys|last|length|lower|nl2br|number_format|raw|reverse|round|sort|striptags|title|trim|upper)(?=[\\s\\|\\]\\}\\):,]|\\.\\.|\\*\\*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255093157888,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }