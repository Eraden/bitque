
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
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)(?=\\s*\\??\\.\\s*prototype\\b(?!\\$))"),
      scope: vec![
        Scope {
            a: 61925366764601344,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*(?:\n  ([\\p{Lu}][_$\\p{Nd}\\p{Lu}]*) |\n  ([_$\\p{L}][_$\\p{L}\\p{N}]*)\n)(?=\\s*\\??\\.\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087305966398,
            b: 65584318113644544,
        },
    ]),(4, vec![
        Scope {
            a: 49259087346401513,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:\n  ([\\p{Lu}][_$\\p{Nd}\\p{Lu}]*) |\n  ([_$\\p{L}][_$\\p{L}\\p{N}]*)\n)(?=\\s*\\??\\.\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087305966398,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 49259087346401431,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }