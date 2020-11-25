
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
      regex: Regex::new("(?x) # multi-line regex definition mode\n(\\#)(?:\n        ([0-9a-fA-F])\n        ([0-9a-fA-F])\n        ([0-9a-fA-F])\n        ([0-9a-fA-F])?\n|       ([0-9a-fA-F]{2})\n        ([0-9a-fA-F]{2})\n        ([0-9a-fA-F]{2})\n        ([0-9a-fA-F]{2})?\n)\\b"),
      scope: vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004945,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136441680405,
            b: 699747414872948736,
        },
    ]),(3, vec![
        Scope {
            a: 59955136441680405,
            b: 700028889849659392,
        },
    ]),(4, vec![
        Scope {
            a: 59955136441680405,
            b: 700310364826370048,
        },
    ]),(5, vec![
        Scope {
            a: 59955136441680405,
            b: 700591839803080704,
        },
    ]),(6, vec![
        Scope {
            a: 59955136441680405,
            b: 699747414872948736,
        },
    ]),(7, vec![
        Scope {
            a: 59955136441680405,
            b: 700028889849659392,
        },
    ]),(8, vec![
        Scope {
            a: 59955136441680405,
            b: 700310364826370048,
        },
    ]),(9, vec![
        Scope {
            a: 59955136441680405,
            b: 700591839803080704,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)                    # multi-line regex definition mode\n(?:-|\\+)?               # negative / positive\n(?:\n    (?:\n        [0-9]+          # integer part\n        (?:\\.[0-9]+)?   # fraction\n    ) |\n    (?:\\.[0-9]+)        # fraction without leading zero\n)\n((?:                    # units\n    px|pt|ch|cm|mm|in|\n    r?em|ex|pc|vw|vh|vmin|vmax|deg|\n    g?rad|turn|dpi|dpcm|dppx|m?s|k?Hz\n)\\b|%)?"),
      scope: vec![
        Scope {
            a: 59955089171873792,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787033309329,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }