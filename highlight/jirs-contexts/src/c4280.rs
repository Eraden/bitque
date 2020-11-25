
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
      regex: Regex::new("(B)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123029,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4197 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(C)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123616,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4198 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(E)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123617,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4199 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(I)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123240,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4200 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(F)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122938,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4201 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(L)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122395,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4202 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(S)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123618,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4203 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(X)(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122950,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4204 }),
    ]),
      with_prototype: None
    }),
]
} }