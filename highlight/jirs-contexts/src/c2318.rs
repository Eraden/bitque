
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
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(true|false|null)\\b"),
      scope: vec![
        Scope {
            a: 59955110639828992,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(this|super)\\b"),
      scope: vec![
        Scope {
            a: 49259061524824064,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])(?x:\n  # 0x1., 0x1.1, 0x1.1p1, 0x1.1p-1, 0x1.p1, 0x1.p-1 | 0x1p1\n  (?:(_*)\\h[\\h_]*?(_*)) (?: (\\.) (?: (?:(_*)\\h[\\h_]*?(_*))? (?:[pP][-+]?(?:(_*)\\d[\\d_]*?(_*)))? \\b )? | (?:[pP][-+]?(?:(_*)\\d[\\d_]*?(_*))) \\b )\n  # 0x.1, 0x.1p1, 0x.1p-1\n  | (\\.) (?:(_*)\\h[\\h_]*?(_*)) (?:[pP][-+]?(?:(_*)\\d[\\d_]*?(_*)))? \\b\n)"),
      scope: vec![
        Scope {
            a: 59955089176592600,
            b: 11258999068426240,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 11258999068426240,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397928,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 47288620735397928,
            b: 0,
        },
    ]),(12, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(13, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(14, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(15, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b\\d[\\d_]*?(_*)\n  (?:\n    # 1., 1.1, 1.1e1, 1.1e-1, 1.e1, 1.e-1, 1.d, 1.1d, 1.1e1d, 1.1e-1d, 1.e1d, 1.e-1d\n    (\\.) (?: (?:(_*)\\d[\\d_]*?(_*))? (?:[eE][-+]?(?:(_*)\\d[\\d_]*?(_*)))? ([dDfF])? \\b )?\n    # 1e1 1e1d\n    | (?:[eE][-+]?(?:(_*)\\d[\\d_]*?(_*))) ([dDfF])? \\b\n    # 1d\n    | ([dDfF]) \\b\n  )\n  # .1, .1e1, .1e-1\n  | (\\.) (?:(_*)\\d[\\d_]*?(_*)) (?:[eE][-+]?(?:(_*)\\d[\\d_]*?(_*)))? ([dDfF])? \\b\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 11258999068426240,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397928,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ]),(12, vec![
        Scope {
            a: 47288620735397928,
            b: 0,
        },
    ]),(13, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(14, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(15, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(16, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(17, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[bB])(_*)[01][01_]*?(_*)([lL])?\\b"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 11258999068426240,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070765,
            b: 11258999068426240,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])(?:(_*)\\h[\\h_]*?(_*))([lL])?\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 11258999068426240,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 11258999068426240,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0)(?:(_+)|[0-7_]+?(_*)|([\\d_]+))([lL])?\\b"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 11258999068426240,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070690,
            b: 11258999068426240,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d[\\d_]*?(_*)([lL])?\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 11258999068426240,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314667667496,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }