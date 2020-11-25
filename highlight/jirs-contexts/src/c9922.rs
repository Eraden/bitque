
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
      regex: Regex::new("(?x)\n\\s*\n(?:([_$\\p{L}][-$\\p{L}\\p{N}.]*)(:))?\n([_$\\p{L}][-$\\p{L}\\p{N}]*)\n(?=\\s|=|/?>|/\\*|//)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183350,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 47288620745621655,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186477183127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }