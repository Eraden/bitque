
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
      regex: Regex::new("(?x:         # turn on extended mode\n  -?         # an optional minus\n  (?:\n    0        # a zero\n    |        # ...or...\n    [1-9]    # a 1-9 character\n    \\d*      # followed by zero or more digits\n  )\n  (?:\n    (?:\n      \\.     # a period\n      \\d+    # followed by one or more digits\n    )?\n    (?:\n      [eE]   # an e character\n      [+-]?  # followed by an option +/-\n      \\d+    # followed by one or more digits\n    )?       # make exponent optional\n  )?         # make decimal portion optional\n)"),
      scope: vec![
        Scope {
            a: 59955089171349504,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }