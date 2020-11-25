
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
      regex: Regex::new("(?x)\n(?:\n  ((?:https?|mailto|ftp|file)  # specify separately so we can mark them as links that TextMate opens\n  ?:{1}  # inline only\n  \\S*)   # (others such as image are partial URLs and/or TextMate cannot handle them)\n|\n  (([a-zA-Z0-9][a-zA-Z0-9_]*)\n  (:{1,2})\n  (\\S*))\n)\n(?:(\\[)([^\\]]*)(\\]))"),
      scope: vec![
        Scope {
            a: 46445308193013760,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114280588597985731,
            b: 23643898043695104,
        },
    ]),(3, vec![
        Scope {
            a: 52636636702572995,
            b: 23643898043695104,
        },
    ]),(4, vec![
        Scope {
            a: 59955200844104131,
            b: 23643898043695104,
        },
    ]),(5, vec![
        Scope {
            a: 114280588622365123,
            b: 23643898043695104,
        },
    ]),(6, vec![
        Scope {
            a: 59955200832963011,
            b: 51228806538592256,
        },
    ]),(7, vec![
        Scope {
            a: 49258876840051139,
            b: 23643898043695104,
        },
    ]),(8, vec![
        Scope {
            a: 59955200832963011,
            b: 48132581794775040,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }