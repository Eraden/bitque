
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
      regex: Regex::new("\\b(define|undef)\\s+((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636705390603,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632974347,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(el)?if\\b"),
      scope: vec![
        Scope {
            a: 52636636705390603,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 303 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(else|endif)\\b"),
      scope: vec![
        Scope {
            a: 52636636705390603,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 397 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(error|warning)\\b\\s*(.*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451949097222144,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(region)\\b\\s*(.*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630090763,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(endregion)\\b\\s*(.*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087303081995,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(line)\\s+(default|hidden)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 397 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(line)\\s+(\\d*)\\s+((\").*(\"))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176461530,
            b: 3096224743817216,
        },
    ]),(3, vec![
        Scope {
            a: 55451420828565515,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956406,
            b: 3096224743817216,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 397 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(pragma)\\s+(checksum)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 304 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(pragma)\\s+(warning)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787029245963,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 306 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*"),
      scope: vec![
        Scope {
            a: 50103314654363648,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }