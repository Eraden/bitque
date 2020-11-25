
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
      regex: Regex::new("\\s*(<)\\s*(const)\\s*(>)"),
      scope: vec![
        Scope {
            a: 105845414570622976,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445243844722838,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46445243844722838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?<!\\+\\+|--)(?<=^return|[^\\._$\\p{L}\\p{N}]return|^throw|[^\\._$\\p{L}\\p{N}]throw|^yield|[^\\._$\\p{L}\\p{N}]yield|^await|[^\\._$\\p{L}\\p{N}]await|^default|[^\\._$\\p{L}\\p{N}]default|[=(,:>*?\\&\\|\\^]|[^_$\\p{L}\\p{N}](?:\\+\\+|\\-\\-)|[^\\+]\\+|[^\\-]\\-))\\s*(<)(?!<?\\=)(?!\\s*(?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445243844722838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9404 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?<=^))\\s*(<)(?=[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445243844722838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9405 }),
    ]),
      with_prototype: None
    }),
]
} }