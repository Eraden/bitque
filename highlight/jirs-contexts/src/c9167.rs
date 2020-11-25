
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
        a: 845039110455296,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845039110455296,
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
      regex: Regex::new("\\b(EQUAL|GREATER|LESS|NONE|SOME|abstraction|abstype|and|andalso|array|as|before|bool|case|char|datatype|do|else|end|eqtype|exception|exn|false|fn|fun|functor|handle|if|in|include|infix|infixr|int|let|list|local|nil|nonfix|not|o|of|op|open|option|orelse|overload|print|raise|real|rec|ref|sharing|sig|signature|string|struct|structure|substring|then|true|type|unit|val|vector|where|while|with|withtype|word)\\b"),
      scope: vec![
        Scope {
            a: 52635833539166208,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[0-9]+\\b"),
      scope: vec![
        Scope {
            a: 59955089171742720,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z]([A-z0-9]*)\\b"),
      scope: vec![
        Scope {
            a: 61925375354011648,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\"(\\\\\"|[^\"])*\""),
      scope: vec![
        Scope {
            a: 55451184592322560,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9164 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\(\\)|=>|::|\\[\\]|->|:>)"),
      scope: vec![
        Scope {
            a: 59955200840892416,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }