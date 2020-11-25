
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
      regex: Regex::new("(?i)(?:\\B[-+]|\\b)0x[0-9a-f]*\\.(\\B|\\b[0-9]+)"),
      scope: vec![
        Scope {
            a: 50103314667668048,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\B[-+]|\\b)0[0-9]+\\.(\\B|\\b[0-9]+)"),
      scope: vec![
        Scope {
            a: 50103314667667810,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:\\B[-+])?\n(?:\n  \\b0b[0-1]*|                 # binary\n  \\b0o[0-7]*|                 # octal\n  \\b0x[0-9a-f]*|              # hex\n  (\n    \\B\\.[0-9]+|               # e.g. .999\n    \\b[0-9]+(\\.[0-9]*)?       # e.g. 999.999, 999. or 999\n  )(e[-+]?[0-9]+)?            # e.g. e+123, E-123\n)"),
      scope: vec![
        Scope {
            a: 59955089165189120,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\B[-+]|\\b)(Infinity)\\b"),
      scope: vec![
        Scope {
            a: 59955110691536939,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }