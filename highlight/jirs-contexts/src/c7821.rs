
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
      regex: Regex::new("(<)([_$a-zA-Z][-$:.\\w]*[$\\w]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230305185792,
            b: 0,
        },
        Scope {
            a: 47288629324153014,
            b: 663436520107016192,
        },
    ]),(2, vec![
        Scope {
            a: 46444230305185792,
            b: 0,
        },
        Scope {
            a: 59392130632124725,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7760 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<"),
      scope: vec![
        Scope {
            a: 50103314666750425,
            b: 663436520107016192,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }