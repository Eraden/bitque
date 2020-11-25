
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 6844 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6859 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6864 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6846 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6847 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6871 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6860 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(do|end|case|for|if|cond|unless|try|receive|fn|defmodule|defp?|defprotocol|defimpl|defrecord|defstruct|defmacrop?|defdelegate|defexception|defoverridable|defguardp?|exit|after|rescue|catch|else|raise|reraise|throw|import|require|alias|use|quote|unquote|super|with)\\b(?![?!:])"),
      scope: vec![
        Scope {
            a: 52636636695166976,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(and|not|or|when|xor|in)\\b"),
      scope: vec![
        Scope {
            a: 52636628105232384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z][a-zA-Z0-9_]*\\b(?!:)"),
      scope: vec![
        Scope {
            a: 59392130632319075,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(nil|true|false)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 59955110643695616,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(__(CALLER|ENV|MODULE|DIR)__)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 49259061528690688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)[a-z]\\w*"),
      scope: vec![
        Scope {
            a: 49259087342010467,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628111458517,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&)\\d+"),
      scope: vec![
        Scope {
            a: 59955136488210531,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628179943523,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&)\\w+(?=\\/\\d|\\()"),
      scope: vec![
        Scope {
            a: 49259087373205603,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628179943523,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\^)[a-z]\\w*"),
      scope: vec![
        Scope {
            a: 49259087433105507,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628239843427,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":\'"),
      scope: vec![
        Scope {
            a: 47288629325004899,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6754 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":\""),
      scope: vec![
        Scope {
            a: 47288629325004899,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6755 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!:)(:)(?>[\\p{L}_][\\w@]*(?>[?!]|=(?![>=]))?|\\<\\>|===?|!==?|<<>>|<<<|>>>|~~~|::|<\\-|\\|>|=>|=~|=|/|\\\\\\\\|\\*\\*?|\\.\\.?\\.?|>=?|<=?|&&?&?|\\+\\+?|\\-\\-?|\\|\\|?\\|?|\\!|@|\\%?\\{\\}|%|\\[\\]|\\^(\\^\\^)?)"),
      scope: vec![
        Scope {
            a: 59955136445349987,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004899,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?>[\\p{L}_][\\w@]*(?>[?!])?)(:)(?!:)"),
      scope: vec![
        Scope {
            a: 59955136548175971,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004899,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(##).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711023108195,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038819,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^[ \\t]+)?(#).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711032873059,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038819,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)\\?(\\\\(x[0-9A-Fa-f]{1,2}(?![0-9A-Fa-f])\\b|[^xMC])|[^\\s\\\\])"),
      scope: vec![
        Scope {
            a: 59955089168859136,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }