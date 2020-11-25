
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
      regex: Regex::new("\\s*(([\'\"]?)(\\s*CSS)(\\2))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 3940911666954240,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4358 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(([\'\"]?)(\\s*HTML)(\\2))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 1407636876558336,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4360 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(([\'\"]?)(\\s*JAVASCRIPT)(\\2))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 12103685991563264,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4362 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(([\'\"]?)(\\s*JSON)(\\2))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 10696311108009984,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4363 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(([\'\"]?)(\\s*SQL)(\\2))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 19422035386040320,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4366 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(([\'\"]?)(\\s*XML)(\\2))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 22518260129857536,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4367 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((\')(\\s*\\b[_\\p{L}]\\w*\\b)(\'))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 563211946426368,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4365 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((\"?)(\\s*\\b[_\\p{L}]\\w*\\b)(\\2))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632123501,
            b: 563211946426368,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4361 }),
        ContextReference::Direct(ContextId { index: 4359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4298 })),
]
} }