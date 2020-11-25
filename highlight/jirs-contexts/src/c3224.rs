
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 114280120506515496,
        b: 262897628247752704,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 114280120506515496,
        b: 262897628247752704,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("(?x:\n  ^             # the beginning of the line\n  [ \\t]*\n  (\n    \\2          # the backtick/tilde combination that opened the code fence\n    (?:\\3|\\4)*  # plus optional additional closing characters\n  )\n  \\s*(?m:$)          # any amount of whitespace until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165227,
            b: 11263010567880704,
        },
    ]),(1, vec![
        Scope {
            a: 47288629330576302,
            b: 48132431470919680,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }