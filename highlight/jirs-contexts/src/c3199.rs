
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3177 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\$?\\("),
      scope: vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3166 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\$?{"),
      scope: vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3167 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\$?[@%<?^+|*]"),
      scope: vec![
        Scope {
            a: 49259061583216688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\$"),
      scope: vec![
        Scope {
            a: 59955200847314992,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)[\\p{L}]"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49258876841754624,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787073613872,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }