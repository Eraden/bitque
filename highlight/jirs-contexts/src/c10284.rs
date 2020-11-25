
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
      regex: Regex::new("(?x:\n  (\n    ((?:\\d{1,3}\\.){3}\\d{1,3})|     # simple IPv4\n    ((?i:[a-f0-9:]+:+)+[a-f0-9]+)| # simple IPv6\n    (\\S+)                          # anything else\n  )\n  [ \\t]*                           # any whitespace\n  (\\S*)                            # anything else on the line\n)"),
      scope: vec![
        Scope {
            a: 46445780657963169,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444217419694241,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089306814621,
            b: 45317471250415616,
        },
    ]),(3, vec![
        Scope {
            a: 59955089306814622,
            b: 45317471250415616,
        },
    ]),(4, vec![
        Scope {
            a: 55451949266370721,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 50103314664194048,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }