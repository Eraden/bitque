
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
      regex: Regex::new("(<)((?:mailto:)?[-+.\\w]+@[-a-z0-9]+(\\.[-a-z0-9]+)*\\.[a-z]+)(>)"),
      scope: vec![
        Scope {
            a: 46443487165153735,
            b: 13792273858822144,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629312815286,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 114280588597985329,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629312815275,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\w.+-]+@[\\w-]+(\\.((?![._-][\\W])[\\w_-])+)+(?![_-])"),
      scope: vec![
        Scope {
            a: 114280588597985329,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }