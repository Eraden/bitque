
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
      regex: Regex::new("\\b(Date|Дата|Data|Datum|Date|@is.po|@it.po|시각|Data|Дата|Datum|Ngày tháng|日期)\\s*(:)\\s*(.*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114281636569612311,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 185492099848142848,
        },
    ]),(3, vec![
        Scope {
            a: 59955110680461333,
            b: 6473924464345088,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }