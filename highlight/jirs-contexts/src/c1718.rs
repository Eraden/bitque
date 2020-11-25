
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
      regex: Regex::new("\\b(On branch|На клон|En la branca|Auf Branch|Sur la branche|Sul branch|현재 브랜치|No ramo|На ветке|På grenen|Trên nhánh|位于分支)\\s+(.*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114281636610965525,
            b: 6473924464345088,
        },
    ]),(2, vec![
        Scope {
            a: 59955110680002581,
            b: 6473924464345088,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }