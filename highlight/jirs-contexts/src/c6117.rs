
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
        a: 46444466403475541,
        b: 24488322973827072,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444466403475541,
        b: 24488322973827072,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Named("preprocessor-comments".to_string())),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)endmacro)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636636717449301,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%%)((?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?:(\\:)?|\\b))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632975938,
            b: 23925746682560512,
        },
    ]),(4, vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%)({))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466403475541,
            b: 24488322973827072,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ]),(3, vec![
        Scope {
            a: 46445029020205143,
            b: 0,
        },
        Scope {
            a: 47288521962160310,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6038 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)(?:rotate))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636628119322709,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)(?:include|depend))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636636717449471,
            b: 23925746682560512,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6118 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)(?:pathsearch))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6118 }),
        ContextReference::Direct(ContextId { index: 6107 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)x?i?define)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466403475541,
            b: 24488322973827072,
        },
        Scope {
            a: 52636636717449471,
            b: 23925746682560512,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6039 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6119 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6106 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6068 })),
]
} }