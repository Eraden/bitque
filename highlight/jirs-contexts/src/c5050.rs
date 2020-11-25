
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
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5034 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bextern\\b"),
      scope: vec![
        Scope {
            a: 52636787017252864,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4984 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=>|[^ \\t\\n\\$=<_+\'(),&:\\[\\]*\\p{L}\\p{N}])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<"),
      scope: vec![
        Scope {
            a: 47288629329985718,
            b: 20266198323167232,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4985 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521944400054,
            b: 20266198323167232,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4986 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5088 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Copy|Send|Sized|Sync|Drop|Fn|FnMut|FnOnce|Box|ToOwned|Clone|PartialEq|PartialOrd|Eq|Ord|AsRef|AsMut|Into|From|Default|Iterator|Extend|IntoIterator|DoubleEndedIterator|ExactSizeIterator|Option|Some|None|Result|Ok|Err|SliceConcatExt|String|ToString|Vec)\\b"),
      scope: vec![
        Scope {
            a: 61925375349358592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288620725829632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\+"),
      scope: vec![
        Scope {
            a: 52636628103462912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bas\\b"),
      scope: vec![
        Scope {
            a: 52636628103462912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }