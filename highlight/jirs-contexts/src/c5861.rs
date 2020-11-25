
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
  prototype: Some(
    ContextId {
        index: 5884,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?"),
      scope: vec![
        Scope {
            a: 47288629327364278,
            b: 22799473113563136,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5838 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?=\n  (?x:\n    [^\\s[-?:,\\[\\]{}#&*!|>\'\"%@`]]\n  | [?:-] \\S\n)\n  (\n      [^\\s:]\n    | : \\S\n    | \\s+ (?![#\\s])\n  )*\n  \\s*\n  :\n  (\\s|(?m:$))\n)\n"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5839 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":(?=\\s|(?m:$))"),
      scope: vec![
        Scope {
            a: 47288620737430065,
            b: 22799473113563136,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }