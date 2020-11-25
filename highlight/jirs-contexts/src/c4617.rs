
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
      regex: Regex::new("\\b(?i)(0x)\\h*(L)"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553278,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(0x)(_?\\h)+"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(0o?)(?=o|[0-7])[0-7]*(L)"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553278,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(0)[0-7]+"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(0o)(_?[0-7])+"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(0b)[01]*(L)"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553278,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(0b)(_?[01])*"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  # 1.j, 1.1j, 1.1e1j, 1.1e-1j, 1.e1j, 1.e-1 | 1e1j, 1e-1j\n  \\b(?:\\d+(?:_\\d+)*) (\\.)? (?:\\d+(?:_\\d+)*)? (?:[eE][-+]?(?:\\d+(?:_\\d+)*))?\n  # .1j, .1e1j, .1e-1j\n  | (\\.) (?:\\d+(?:_\\d+)*) (?:[eE][-+]?(?:\\d+(?:_\\d+)*))?\n)([jJ])"),
      scope: vec![
        Scope {
            a: 59955089199268058,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397950,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397950,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553278,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  # 1., 1.1, 1.1e1, 1.1e-1, 1.e1, 1.e-1 | 1e1, 1e-1\n  \\b(?:\\d+(?:_\\d+)*) (?: (\\.) (?:\\d+(?:_\\d+)*)? (?:[eE][-+]?(?:\\d+(?:_\\d+)*))? | (?:[eE][-+]?(?:\\d+(?:_\\d+)*)) )\n  # .1, .1e1, .1e-1\n  | (\\.) (?:\\d+(?:_\\d+)*) (?:[eE][-+]?(?:\\d+(?:_\\d+)*))?\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397950,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397950,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(?:[1-9]\\d*|0)(L)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476553278,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)([1-9][\\d_]*|0)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }