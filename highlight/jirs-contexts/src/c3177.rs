
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
      regex: Regex::new("(\\$\\$?\\()(call)\\s"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882988892160,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ]),(2, vec![
        Scope {
            a: 59955110653329456,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3138 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$\\$?\\()(patsubst|filter)\\s"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882988892160,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ]),(2, vec![
        Scope {
            a: 61925255090602032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3140 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$\\$?\\()(wildcard)\\s"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882988892160,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ]),(2, vec![
        Scope {
            a: 61925255090602032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3141 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$\\$?\\()(info|warning|error)\\s"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882988892160,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ]),(2, vec![
        Scope {
            a: 61925255090602032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3142 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)   # ignore whitespace\n(\\$\\$?\\()\n  (\n    subst|\n    strip|\n    findstring|\n    filter-out|\n    sort|\n    word|\n    wordlist|\n    words|\n    firstword|\n    lastword|\n    dir|\n    notdir|\n    suffix|\n    basename|\n    addsuffix|\n    addprefix|\n    join|\n    realpath|\n    abspath|\n    if|\n    or|\n    and|\n    foreach|\n    file|\n    value|\n    eval|\n    origin|\n    flavor|\n    guile\n  )\n  \\s\n"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882988892160,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ]),(2, vec![
        Scope {
            a: 61925255090602032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3183 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$\\$?\\()(shell)\\s"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787027148982,
            b: 13510798882111488,
        },
    ]),(2, vec![
        Scope {
            a: 61925255090602032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3143 }),
    ]),
      with_prototype: None
    }),
]
} }