
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
      regex: Regex::new("\\]"),
      scope: vec![
        Scope {
            a: 47288629365833899,
            b: 12384898975268864,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?:\n  (\\\\[wWsSdD]|\\.)|\n  (\\\\(?:[trnvf0]|c[A-Z]|x[\\da-fA-F]{2}|u[\\da-fA-F]{4}|.))|\n  .\n)\n(\\-)\n(?:\n  (\\\\[wWsSdD]|\\.)|\n  (\\\\(?:[trnvf0]|c[A-Z]|x[\\da-fA-F]{2}|u[\\da-fA-F]{4}|.))|\n  [^]]\n)"),
      scope: vec![
        Scope {
            a: 59955136461799697,
            b: 12384898975268864,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136461799665,
            b: 218987720859451392,
        },
    ]),(2, vec![
        Scope {
            a: 59955200847315722,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 47288629328937004,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59955136461799665,
            b: 218987720859451392,
        },
    ]),(5, vec![
        Scope {
            a: 59955200847315722,
            b: 12384898975268864,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2723 })),
]
} }