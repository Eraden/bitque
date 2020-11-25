
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(await)(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636636701196439,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(yield)(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))(?=\\s*\\/\\*([^\\*]|(\\*[^\\/]))*\\*\\/\\s*\\*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196439,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9739 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(yield)(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))(?:\\s*(\\*))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196439,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638522221068439,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))delete(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636628121291250,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))in(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))(?!\\()"),
      scope: vec![
        Scope {
            a: 52636628121289974,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))of(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))(?!\\()"),
      scope: vec![
        Scope {
            a: 52636628121291251,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))instanceof(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636628121289445,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))new(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636628113752215,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9994 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))void(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636628121289448,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(as)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636736848023,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9740 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![
        Scope {
            a: 52636628152877207,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*=|(?<!\\()/=|%=|\\+=|\\-="),
      scope: vec![
        Scope {
            a: 52636628111132041,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\&=|\\^=|<<=|>>=|>>>=|\\|="),
      scope: vec![
        Scope {
            a: 52636628111132041,
            b: 159596960335003648,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<|>>>|>>"),
      scope: vec![
        Scope {
            a: 52636628135904695,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("===|!==|==|!="),
      scope: vec![
        Scope {
            a: 52636628119257239,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<=|>=|<>|<|>"),
      scope: vec![
        Scope {
            a: 52636628152483991,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\!|&&|\\|\\|"),
      scope: vec![
        Scope {
            a: 52636628114800791,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\&|~|\\^|\\|"),
      scope: vec![
        Scope {
            a: 52636628135903383,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\="),
      scope: vec![
        Scope {
            a: 52636628111130775,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("--"),
      scope: vec![
        Scope {
            a: 52636628265730199,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\+\\+"),
      scope: vec![
        Scope {
            a: 52636628265795735,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%|\\*|/|-|\\+"),
      scope: vec![
        Scope {
            a: 52636628119191703,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=[_$\\p{L}\\p{N})\\]])\\s*(/)(?![/*])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628119191703,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }