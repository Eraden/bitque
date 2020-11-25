
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:0_*|[1-9][0-9_]*)?(?:(\\.)(?:\\d[\\d_]*)?)?(?:[eE][-+]??\\d+)?([fFL]?i)"),
      scope: vec![
        Scope {
            a: 59955089199268058,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (?:0_*|[1-9][0-9_]*)\n  (?:\n    (?:\n      (\\.)\n      (?:\n        # 1.1, 1.1e1, 1.1e-1, 1.1f, 1.1e1f, 1.1e-1f, 1.1L, 1.1e1L, 1.1e-1L\n        (?:\\d[\\d_]*) (?:[eE][-+]??\\d+)?\n        # 1.e1, 1.e-1, 1.e1f, 1.e-1f, 1.e1L, 1.e-1L\n        | (?:[eE][-+]??\\d+)\n        # 1., 1.f, 1.L # but not `..` or method/attribute access\n        | (?!\\.|\\s*[\\p{L}_]) )\n      # 1e1 1e1f 1e1L\n      | (?:[eE][-+]??\\d+)\n    ) ([fFL])?\n    # 1f\n    | ([fF])\n  )\n  # .1, .1e1, .1e-1, .1f, .1e1f, .1e-1f, .1L, .1e1L, .1e-1L\n  | (\\.) (?:\\d[\\d_]*) (?:[eE][-+]??\\d+)? ([fFL])?\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397904,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[xX])(_?)(?:\\h[\\h_]*)?(?:(\\.)(?:\\h[\\h_]*)?)?(?:[pP][-+]??\\d+)?([fFL]?i)"),
      scope: vec![
        Scope {
            a: 59955089199268056,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667472,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620735397904,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[xX])(_?)(?:\\h[\\h_]*)?(?:(\\.)(?:\\h[\\h_]*)?)?(?:[pP][-+]??\\d+)([fFL])?"),
      scope: vec![
        Scope {
            a: 59955089176592600,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667472,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620735397904,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[bB])(_?)(?:[01][01_]*)([fFL]?i)"),
      scope: vec![
        Scope {
            a: 59955089199268269,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667472,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[bB])(_?)(?:[01][01_]*)([fF])"),
      scope: vec![
        Scope {
            a: 59955089176592813,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667472,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }