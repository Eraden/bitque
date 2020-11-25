
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
        a: 114282585759219712,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 114282585759219712,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n    [ \\t]*\\*{4,}    # if there are more than 3 its not applicable to be bold or italic\n|   [ \\t]+\\*(?!\\*)  # whitespace followed by 1 is also not applicable (but whitespace followed by 2 could be bold punctuation)\n|   ^\\*(?!\\*)       # emphasis can\'t be closed at the start of the line"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*)(\\*\\*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114282585759219712,
            b: 0,
        },
        Scope {
            a: 47288629368193195,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 47288629354365099,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*\\*"),
      scope: vec![
        Scope {
            a: 47288629354365099,
            b: 13792273858822144,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3212 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 47288629368193195,
            b: 13792273858822144,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3213 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3337 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3312 })),
]
} }