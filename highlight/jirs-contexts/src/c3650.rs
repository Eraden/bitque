
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46443865082167296,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46443865082167296,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 46443865082167296,
            b: 0,
        },
        Scope {
            a: 47288521944400043,
            b: 15762598695796736,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(align|allocate|code_seg|deprecated|property|uuid)\\b\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439027245056,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865082167296,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 15762598695796736,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3651 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(appdomain|deprecated|dllimport|dllexport|jintrinsic|naked|noalias|noinline|noreturn|nothrow|novtable|process|restrict|safebuffers|selectany|thread)\\b"),
      scope: vec![
        Scope {
            a: 59955136410681344,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }