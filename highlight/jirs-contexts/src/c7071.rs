
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
      regex: Regex::new("\\(\\)"),
      scope: vec![
        Scope {
            a: 59955110657982569,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b-?[0-9][0-9_]*((\\.([0-9][0-9_]*([eE][+-]??[0-9][0-9_]*)?)?)|([eE][+-]??[0-9][0-9_]*))"),
      scope: vec![
        Scope {
            a: 59955089232494697,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(-?((0(x|X)[0-9a-fA-F][0-9a-fA-F_]*)|(0(o|O)[0-7][0-7_]*)|(0(b|B)[01][01_]*)|([0-9][0-9_]*)))"),
      scope: vec![
        Scope {
            a: 59955089176463524,
            b: 29554872554618880,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(true|false|null|unit)\\b"),
      scope: vec![
        Scope {
            a: 59963674808877056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }