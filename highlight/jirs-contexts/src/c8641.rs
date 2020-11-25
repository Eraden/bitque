
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
        a: 844991865815040,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844991865815040,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 8656,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 8665 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8655 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8640 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8651 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8631 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8643 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8633 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8663 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(parsed)?\\s*(class)\\s+\\b([A-Za-z][A-Za-z0-9_]*)\\b\\s+{"),
      scope: vec![
        Scope {
            a: 46453880979718276,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439032225792,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576500015236,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632319108,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8613 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(group)\\s+\\b([A-Za-z][A-Za-z0-9_]*)\\b\\s+=\\s+(\\d+)\\s+{"),
      scope: vec![
        Scope {
            a: 46443865087148032,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576471179264,
            b: 0,
        },
        Scope {
            a: 50104723410452612,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087332114564,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089171021824,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8614 }),
    ]),
      with_prototype: None
    }),
]
} }