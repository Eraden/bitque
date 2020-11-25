
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
      regex: Regex::new("(?=\\s*extends|(?m:$)|\\{)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:[a-z]\\w*.)*[A-Z]\\w*)\\s*(?:(,)|(?m:$)|\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186470433181,
            b: 9288674231451648,
        },
    ]),(2, vec![
        Scope {
            a: 47288629358232273,
            b: 54043337262366720,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }