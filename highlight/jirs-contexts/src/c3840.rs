
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
        a: 46443865082232832,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46443865082232832,
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
            a: 46443865082232832,
            b: 0,
        },
        Scope {
            a: 47288521944400043,
            b: 16044073672507392,
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
            a: 48414439027310592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865082232832,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 16044073672507392,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3841 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(appdomain|deprecated|dllimport|dllexport|jintrinsic|naked|noalias|noinline|noreturn|nothrow|novtable|process|restrict|safebuffers|selectany|thread)\\b"),
      scope: vec![
        Scope {
            a: 59955136410746880,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }