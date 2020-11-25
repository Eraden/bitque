
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
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (?: ((?xi:[A-Za-z_][A-Za-z_0-9]* \\s* (?=:))) : )?\n  \\s* (block) \\b\n  \\s* ((?xi:!.*))?)\n  (?m:$)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632124632,
            b: 631911322715422720,
        },
    ]),(2, vec![
        Scope {
            a: 52636636703295685,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 51510711159160832,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }