
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
      regex: Regex::new("(?<!<)(<<)([^,]*?)((,\\s*)(.*?))?(>>)(?!<)"),
      scope: vec![
        Scope {
            a: 46450187275862016,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200935526582,
            b: 23643898043695104,
        },
    ]),(2, vec![
        Scope {
            a: 114280588700288563,
            b: 23643898043695104,
        },
    ]),(5, vec![
        Scope {
            a: 49258876942614612,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 59955200935526571,
            b: 23643898043695104,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }