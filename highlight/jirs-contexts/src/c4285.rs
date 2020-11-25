
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
      regex: Regex::new("(?ix:\n  (\") ( [-+]? (?:\n  (\\.)[\\d_]+ (?: e[-+]?[\\d_]+ )?  |  # .1 .1e1 .1e-1 .1e+1\n      [\\d_]+ (?: (\\.) (?:\n      [\\d_]+ (?: e[-+]?[\\d_]+ )?  |  # 1.1 1.1e1 1.1e-1 1.1e+1\n                 e[-+]?[\\d_]+ )?  |  # 1. 1.e1 1.e-1 1.e+1\n                 e[-+]?[\\d_]+ )      # 1e1 1e-1 1e+1\n  ) ) (\")\n)"),
      scope: vec![
        Scope {
            a: 46444217269813248,
            b: 0,
        },
        Scope {
            a: 55451420828565565,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176592602,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?ix:\n  (\') ( [-+]? (?:\n  (\\.)[\\d_]+ (?: e[-+]?[\\d_]+ )?  |  # .1 .1e1 .1e-1 .1e+1\n      [\\d_]+ (?: (\\.) (?:\n      [\\d_]+ (?: e[-+]?[\\d_]+ )?  |  # 1.1 1.1e1 1.1e-1 1.1e+1\n                 e[-+]?[\\d_]+ )?  |  # 1. 1.e1 1.e-1 1.e+1\n                 e[-+]?[\\d_]+ )      # 1e1 1e-1 1e+1\n  ) ) (\')\n)"),
      scope: vec![
        Scope {
            a: 46444217269813248,
            b: 0,
        },
        Scope {
            a: 55451420831973437,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176592602,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?ix:\n  (\\.)[\\d_]+ (?: e[-+]?[\\d_]+   )? \\b |  # .1 .1e1 .1e-1 .1e+1\n    \\b[\\d_]+ (?: (\\.) (?: (?:\n      [\\d_]+ (?: e[-+]?[\\d_]+   )? \\b |  # 1.1 1.1e1 1.1e-1 1.1e+1\n                 e[-+]?[\\d_]+\\b )     |  # 1.e1 1.e-1 1.e+1\n                 (?=[^.]))            |  # 1. (protect the .. operator)\n                 e[-+]?[\\d_]+\\b )        # 1e1 1e-1 1e+1\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\")([-+]?[\\d_]+)(\")"),
      scope: vec![
        Scope {
            a: 46444217269813248,
            b: 0,
        },
        Scope {
            a: 55451420828565565,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176461530,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\')([-+]?[\\d_]+)(\')"),
      scope: vec![
        Scope {
            a: 46444217269813248,
            b: 0,
        },
        Scope {
            a: 55451420831973437,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176461530,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[bB])[01_]+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070765,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])[\\h_]+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0)[0-7_]+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070690,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\d_]+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }