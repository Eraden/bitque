
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
  prototype: Some(
    ContextId {
        index: 5884,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*)([^\\s\\[\\]/{/},]+)([^\\s\\]},]\\S*)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196737,
            b: 22799473113563136,
        },
        Scope {
            a: 47288629340471377,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087321432145,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314669372229,
            b: 22799473113563136,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }