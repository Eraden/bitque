
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
      regex: Regex::new("(?xi:^ \\s* (implicit) \\s+ (?=none|(?xi)\n(?:\n  (?xi)\n(?:\n  \\bCHARACTER\\b \\s* (?xi)\n(?:\n  \\(\n    \\s*\n    (?:\n      LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* , \\s* KIND \\s* = \\s* (?xi:\\d+|\\w+)\n    |\n      (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* ,  \\s* (?:KIND \\s* =)?  \\s* (?xi:\\d+|\\w+)\n    |\n      KIND \\s* = \\s* (?xi:\\d+|\\w+)  \\s* (?: , \\s* LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n)?\n    |\n      (?:LEN \\s* = \\s* )? (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n\n    )\n    \\s*\n  \\)\n|\n  \\s* \\* \\s* (?xi)\n(?:\n  (?xi:\\d+)\n|\n  \\( (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\)\n)\n \\s* ,? \\s*\n)\n\n|\n  \\b(?:COMPLEX|INTEGER|LOGICAL|REAL)\\b (?:\\s* (?xi)\n(?:\n  \\s*\n  (?:\n    \\(  (?:\\s* kind \\s* = \\s* )?  (?xi:\\d+|\\w+) \\)\n  |\n    \\* \\s* (?xi:\\d+)\n  )\n  \\s*\n)\n)?\n|\n  \\b DOUBLE \\s+ (?:COMPLEX|PRECISION) \\b\n)\n\n|\n  type \\s* \\( \\s* (?xi)\n(?:\n  \\bCHARACTER\\b \\s* (?xi)\n(?:\n  \\(\n    \\s*\n    (?:\n      LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* , \\s* KIND \\s* = \\s* (?xi:\\d+|\\w+)\n    |\n      (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* ,  \\s* (?:KIND \\s* =)?  \\s* (?xi:\\d+|\\w+)\n    |\n      KIND \\s* = \\s* (?xi:\\d+|\\w+)  \\s* (?: , \\s* LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n)?\n    |\n      (?:LEN \\s* = \\s* )? (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n\n    )\n    \\s*\n  \\)\n|\n  \\s* \\* \\s* (?xi)\n(?:\n  (?xi:\\d+)\n|\n  \\( (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\)\n)\n \\s* ,? \\s*\n)\n\n|\n  \\b(?:COMPLEX|INTEGER|LOGICAL|REAL)\\b (?:\\s* (?xi)\n(?:\n  \\s*\n  (?:\n    \\(  (?:\\s* kind \\s* = \\s* )?  (?xi:\\d+|\\w+) \\)\n  |\n    \\* \\s* (?xi:\\d+)\n  )\n  \\s*\n)\n)?\n|\n  \\b DOUBLE \\s+ (?:COMPLEX|PRECISION) \\b\n)\n \\s* \\)\n|\n  type \\s* \\( \\s* (?xi)\n(?:\n  [A-Za-z_][A-Za-z_0-9]*\n)\n \\s* \\)\n|\n  class \\s* \\( \\s* (?xi)\n(?:\n  [A-Za-z_][A-Za-z_0-9]*\n)\n \\s* \\)\n|\n  class \\s* \\( \\s* \\* \\s* \\)\n)\n))"),
      scope: vec![
        Scope {
            a: 46453258319101952,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636834498757,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7349 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^ \\s* implicit \\s+ (.*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314653642752,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }