
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
        a: 46444204391792892,
        b: 9288674231451648,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444204391792892,
        b: 9288674231451648,
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
            a: 47288629327560875,
            b: 9288674231451648,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\s*\n  (\n    (?:boolean|byte|char|short|int|float|long|double|(?:\\w+\\.)*[A-Z]\\w*\\b(?:<(?:[\\w, ]*)>)?(?:\\[\\s*\\])*)\n  )?\n  \\s*\n  ([a-z_][A-Za-z0-9_]*) # variable"),
      scope: vec![
        Scope {
            a: 46444204391792816,
            b: 9288674231451648,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576474062881,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876840771584,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(boolean|byte|char|short|int|float|long|double|(?:\\w+\\.)*[A-Z]\\w*\\b(?:<(?:[\\w, ]*)>)?(?:\\[\\s*\\])*)"),
      scope: vec![
        Scope {
            a: 46444204391792816,
            b: 9288674231451648,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576474062881,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288629327560896,
            b: 9288674231451648,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2016 })),
]
} }