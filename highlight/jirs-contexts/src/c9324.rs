
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
      regex: Regex::new("(?x)\n  (?<!\\w)\n    0x[0-9A-Fa-f](?:[0-9A-Fa-f]|_[0-9A-Fa-f])*\n    |0o[0-7](?:[0-7]|_[0-7])*\n    |0b[0-1](?:[0-1]|_[0-1])*\n    |([\\+\\-]?) (?: [0-9] | [1-9] (?: [0-9] | _ [0-9] )+ )\n  (?!\\w)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59955089176461460,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636628119191700,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }