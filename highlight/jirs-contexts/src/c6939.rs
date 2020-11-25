
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
        a: 281496458100837,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281496458100837,
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
      regex: Regex::new("^(::) "),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6932 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)           # Minimally modified `file_regex` from `Elm Make.sublime-build`\n^\\-\\-[ ]     # Leading delimiter\n((error)     # \\2: error\n|(warning)   # \\3: warning\n|\\w+         # \\1: any (?m:$)type\n)[:][ ]      # separator\n(.+)         # \\4: tag\n[ ][-][ ]    # separator\n(.+?):       # \\5: (?m:$)file\n(\\d+):       # \\6: (?m:$)line\n(\\d+)        # \\7: (?m:$)column\n\\n(?m:$)          # End"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 114281636672045157,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 61925409717813349,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314671272037,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50104723428606053,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925409717813349,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 114280588597985381,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 59955089168990208,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 59955089168990208,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6933 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\["),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6936 }),
    ]),
      with_prototype: None
    }),
]
} }