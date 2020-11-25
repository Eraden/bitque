
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
      regex: Regex::new("\\b(?i:null|t|single-float-negative-epsilon|single-float-epsilon|short-float-negative-epsilon|short-float-epsilon|pi|nil|multiple-values-limit|most-positive-single-float|most-positive-short-float|most-positive-long-float|most-positive-fixnum|most-positive-double-float|most-negative-single-float|most-negative-short-float|most-negative-long-float|most-negative-fixnum|most-negative-double-float|long-float-negative-epsilon|long-float-epsilon|least-positive-single-float|least-positive-short-float|least-positive-normalized-single-float|least-positive-normalized-short-float|least-positive-normalized-long-float|least-positive-normalized-double-float|least-positive-long-float|least-positive-double-float|least-negative-single-float|least-negative-short-float|least-negative-normalized-single-float|least-negative-normalized-short-float|least-negative-normalized-long-float|least-negative-normalized-double-float|least-negative-long-float|least-negative-double-float|lambda-parameters-limit|lambda-list-keywords|internal-time-units-per-second|double-float-negative-epsilon|double-float-epsilon|char-super-bit|char-meta-bit|char-hyper-bit|char-font-limit|char-control-bit|char-code-limit|char-bits-limit|call-arguments-limit|array-total-size-limit|array-rank-limit|array-dimension-limit)\\b"),
      scope: vec![
        Scope {
            a: 59955110640222208,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#)(\\w|[\\\\+-=<>\'\"&#])+"),
      scope: vec![
        Scope {
            a: 59955200834535424,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004846,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }