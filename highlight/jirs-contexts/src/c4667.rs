
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
      regex: Regex::new("\\b(pi|letters|LETTERS|month\\.abb|month\\.name)\\b"),
      scope: vec![
        Scope {
            a: 61925409790492736,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(TRUE|FALSE|NULL|NA|NA_integer_|NA_real_|NA_complex_|NA_character_|Inf|NaN)\\b"),
      scope: vec![
        Scope {
            a: 59955110641401856,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])\\h+(?:(i)|(I))\\b"),
      scope: vec![
        Scope {
            a: 59955089199268056,
            b: 18014398509481984,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 18014398509481984,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553280,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\.)\\d+|\\b\\d+(\\.)?\\d*)(?:[eE][-+]?\\d+)?(?:(i)|(I))\\b"),
      scope: vec![
        Scope {
            a: 59955089199268058,
            b: 18014398509481984,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397952,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397952,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553280,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 50103314667667520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])\\h+(?:(L)|(l))\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 18014398509481984,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 18014398509481984,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553280,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+(\\.)?\\d*(?:(L)|(l))\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 18014398509481984,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397952,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553280,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])\\h+\\b"),
      scope: vec![
        Scope {
            a: 59955089176592600,
            b: 18014398509481984,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 18014398509481984,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  # 1., 1.1, 1.1e1, 1.1e-1, 1.e1, 1.e-1 | 1, 1e1, 1e-1\n  \\b\\d+ (?: (\\.) (?: \\d* (?:[eE][-+]?\\d+)? \\b )? | (?:[eE][-+]?\\d+)? \\b )\n  # .1, .1e1, .1e-1\n  | (\\.) \\d+ (?:[eE][-+]?\\d+)? \\b\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 18014398509481984,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397952,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397952,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }