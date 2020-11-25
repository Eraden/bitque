
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
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (program)\n  \\s+ ([A-Za-z_][A-Za-z_0-9]*)\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636836071621,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630617289,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (type)\n  (\\s*::\\s*|\\s+)\n  ([A-Za-z_][A-Za-z_0-9]*)\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636701260316672,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628250790085,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615245,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (module)\n  \\s+  (\\w+)\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636738685125,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615803,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (submodule)\n  \\s* \\( ([A-Za-z_][A-Za-z_0-9]*) \\)\n  \\s* ([A-Za-z_][A-Za-z_0-9]*)\n  \\s*\n  (?m:$)\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52645789411115008,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087342012613,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615803,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:\n  ^ \\s*\n  (interface) \\b\n  \\s* ([A-Za-z_][A-Za-z_0-9]*)?\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636715747525,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630617285,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:^\n  \\s*\n  ((?:(?xi:elemental|pure|impure|recursive|module)\\s+)*)\n  ((?xi)\n(?:\n  (?xi)\n(?:\n  \\bCHARACTER\\b \\s* (?xi)\n(?:\n  \\(\n    \\s*\n    (?:\n      LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* , \\s* KIND \\s* = \\s* (?xi:\\d+|\\w+)\n    |\n      (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* ,  \\s* (?:KIND \\s* =)?  \\s* (?xi:\\d+|\\w+)\n    |\n      KIND \\s* = \\s* (?xi:\\d+|\\w+)  \\s* (?: , \\s* LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n)?\n    |\n      (?:LEN \\s* = \\s* )? (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n\n    )\n    \\s*\n  \\)\n|\n  \\s* \\* \\s* (?xi)\n(?:\n  (?xi:\\d+)\n|\n  \\( (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\)\n)\n \\s* ,? \\s*\n)\n\n|\n  \\b(?:COMPLEX|INTEGER|LOGICAL|REAL)\\b (?:\\s* (?xi)\n(?:\n  \\s*\n  (?:\n    \\(  (?:\\s* kind \\s* = \\s* )?  (?xi:\\d+|\\w+) \\)\n  |\n    \\* \\s* (?xi:\\d+)\n  )\n  \\s*\n)\n)?\n|\n  \\b DOUBLE \\s+ (?:COMPLEX|PRECISION) \\b\n)\n\n|\n  type \\s* \\( \\s* (?xi)\n(?:\n  \\bCHARACTER\\b \\s* (?xi)\n(?:\n  \\(\n    \\s*\n    (?:\n      LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* , \\s* KIND \\s* = \\s* (?xi:\\d+|\\w+)\n    |\n      (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* ,  \\s* (?:KIND \\s* =)?  \\s* (?xi:\\d+|\\w+)\n    |\n      KIND \\s* = \\s* (?xi:\\d+|\\w+)  \\s* (?: , \\s* LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n)?\n    |\n      (?:LEN \\s* = \\s* )? (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n\n    )\n    \\s*\n  \\)\n|\n  \\s* \\* \\s* (?xi)\n(?:\n  (?xi:\\d+)\n|\n  \\( (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\)\n)\n \\s* ,? \\s*\n)\n\n|\n  \\b(?:COMPLEX|INTEGER|LOGICAL|REAL)\\b (?:\\s* (?xi)\n(?:\n  \\s*\n  (?:\n    \\(  (?:\\s* kind \\s* = \\s* )?  (?xi:\\d+|\\w+) \\)\n  |\n    \\* \\s* (?xi:\\d+)\n  )\n  \\s*\n)\n)?\n|\n  \\b DOUBLE \\s+ (?:COMPLEX|PRECISION) \\b\n)\n \\s* \\)\n|\n  type \\s* \\( \\s* (?xi)\n(?:\n  [A-Za-z_][A-Za-z_0-9]*\n)\n \\s* \\)\n|\n  class \\s* \\( \\s* (?xi)\n(?:\n  [A-Za-z_][A-Za-z_0-9]*\n)\n \\s* \\)\n|\n  class \\s* \\( \\s* \\* \\s* \\)\n)\n\\s+)?\n  \\b (FUNCTION|SUBROUTINE) \\b\n  \\s+ ([A-Za-z_][A-Za-z_0-9]*)\n  \\s* (\\(|&) # match opening bracket\n)\n"),
      scope: vec![
        Scope {
            a: 46453344218447872,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636563821363200,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636700281029,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630617285,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7386 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:^\n  \\s*\n  ((?:(?xi:elemental|pure|impure|recursive|module)\\s+)*)\n  ((?xi)\n(?:\n  (?xi)\n(?:\n  \\bCHARACTER\\b \\s* (?xi)\n(?:\n  \\(\n    \\s*\n    (?:\n      LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* , \\s* KIND \\s* = \\s* (?xi:\\d+|\\w+)\n    |\n      (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* ,  \\s* (?:KIND \\s* =)?  \\s* (?xi:\\d+|\\w+)\n    |\n      KIND \\s* = \\s* (?xi:\\d+|\\w+)  \\s* (?: , \\s* LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n)?\n    |\n      (?:LEN \\s* = \\s* )? (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n\n    )\n    \\s*\n  \\)\n|\n  \\s* \\* \\s* (?xi)\n(?:\n  (?xi:\\d+)\n|\n  \\( (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\)\n)\n \\s* ,? \\s*\n)\n\n|\n  \\b(?:COMPLEX|INTEGER|LOGICAL|REAL)\\b (?:\\s* (?xi)\n(?:\n  \\s*\n  (?:\n    \\(  (?:\\s* kind \\s* = \\s* )?  (?xi:\\d+|\\w+) \\)\n  |\n    \\* \\s* (?xi:\\d+)\n  )\n  \\s*\n)\n)?\n|\n  \\b DOUBLE \\s+ (?:COMPLEX|PRECISION) \\b\n)\n\n|\n  type \\s* \\( \\s* (?xi)\n(?:\n  \\bCHARACTER\\b \\s* (?xi)\n(?:\n  \\(\n    \\s*\n    (?:\n      LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* , \\s* KIND \\s* = \\s* (?xi:\\d+|\\w+)\n    |\n      (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\s* ,  \\s* (?:KIND \\s* =)?  \\s* (?xi:\\d+|\\w+)\n    |\n      KIND \\s* = \\s* (?xi:\\d+|\\w+)  \\s* (?: , \\s* LEN \\s* = \\s* (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n)?\n    |\n      (?:LEN \\s* = \\s* )? (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n\n    )\n    \\s*\n  \\)\n|\n  \\s* \\* \\s* (?xi)\n(?:\n  (?xi:\\d+)\n|\n  \\( (?xi:)\n(?:\n  (?xi:TODO_NOT_IMPLEMENTED|\\d+)\n|\n  [A-Za-z_][A-Za-z_0-9]* # hack\n|\n  \\*\n|\n  :\n)\n \\)\n)\n \\s* ,? \\s*\n)\n\n|\n  \\b(?:COMPLEX|INTEGER|LOGICAL|REAL)\\b (?:\\s* (?xi)\n(?:\n  \\s*\n  (?:\n    \\(  (?:\\s* kind \\s* = \\s* )?  (?xi:\\d+|\\w+) \\)\n  |\n    \\* \\s* (?xi:\\d+)\n  )\n  \\s*\n)\n)?\n|\n  \\b DOUBLE \\s+ (?:COMPLEX|PRECISION) \\b\n)\n \\s* \\)\n|\n  type \\s* \\( \\s* (?xi)\n(?:\n  [A-Za-z_][A-Za-z_0-9]*\n)\n \\s* \\)\n|\n  class \\s* \\( \\s* (?xi)\n(?:\n  [A-Za-z_][A-Za-z_0-9]*\n)\n \\s* \\)\n|\n  class \\s* \\( \\s* \\* \\s* \\)\n)\n\\s+)?\n  \\b (FUNCTION|SUBROUTINE) \\b\n  \\s+ ([A-Za-z_][A-Za-z_0-9]*)\n  \\s*\n)\n"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453344218447872,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636563821363200,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636700281029,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630617285,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }