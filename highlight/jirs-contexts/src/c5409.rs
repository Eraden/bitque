
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
        a: 46443865171558576,
        b: 20829148276588544,
    },
    Scope {
        a: 49259087310291018,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46443865171558576,
        b: 20829148276588544,
    },
    Scope {
        a: 49259087310291018,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5407 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[@*]?/)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5337 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\:?[-+=?])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5339 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=@?:)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5340 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\#(?=})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([@*])?(\\#\\#?|%%?|\\^\\^?|,,?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259061527052288,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628191805514,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5344 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([@*]?)(@)([QEPAa])(?=})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259061527052288,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628191805514,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258876864364618,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5373 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[*@](?=})"),
      scope: vec![
        Scope {
            a: 49259061527052288,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }