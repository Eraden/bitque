
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
      regex: Regex::new("\\b(d)\\s*(,)\\s*(drop)\\s*(=)\\s*(.+)"),
      scope: vec![
        Scope {
            a: 46446339025010709,
            b: 8444249301319680,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100252340,
            b: 5911103359942656,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172821,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 52636628100252340,
            b: 5911103359942656,
        },
    ]),(4, vec![
        Scope {
            a: 47288620757877339,
            b: 5911103359942656,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097877534,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(e)\\s*(,)\\s*(edit)\\s*(=)\\s*(.+)"),
      scope: vec![
        Scope {
            a: 46446339025076245,
            b: 8444249301319680,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100252341,
            b: 5911103359942656,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172821,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 52636628100252341,
            b: 5911103359942656,
        },
    ]),(4, vec![
        Scope {
            a: 47288620757877339,
            b: 5911103359942656,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097877534,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(x)\\s*(,)\\s*(exec)\\s*(=)\\s*(.+)"),
      scope: vec![
        Scope {
            a: 46446339025141781,
            b: 8444249301319680,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100252342,
            b: 5911103359942656,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172821,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 52636628100252342,
            b: 5911103359942656,
        },
    ]),(4, vec![
        Scope {
            a: 47288620757877339,
            b: 5911103359942656,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097877534,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(f)\\s*(,)\\s*(fixup)\\s*(=)\\s*(.+)"),
      scope: vec![
        Scope {
            a: 46446339025207317,
            b: 8444249301319680,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100252343,
            b: 5911103359942656,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172821,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 52636628100252343,
            b: 5911103359942656,
        },
    ]),(4, vec![
        Scope {
            a: 47288620757877339,
            b: 5911103359942656,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097877534,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(p)\\s*(,)\\s*(pick)\\s*(=)\\s*(.+)"),
      scope: vec![
        Scope {
            a: 46446339025272853,
            b: 8444249301319680,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100252344,
            b: 5911103359942656,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172821,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 52636628100252344,
            b: 5911103359942656,
        },
    ]),(4, vec![
        Scope {
            a: 47288620757877339,
            b: 5911103359942656,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097877534,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(r)\\s*(,)\\s*(reword)\\s*(=)\\s*(.+)"),
      scope: vec![
        Scope {
            a: 46446339025338389,
            b: 8444249301319680,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100252345,
            b: 5911103359942656,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172821,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 52636628100252345,
            b: 5911103359942656,
        },
    ]),(4, vec![
        Scope {
            a: 47288620757877339,
            b: 5911103359942656,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097877534,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(s)\\s*(,)\\s*(squash)\\s*(=)\\s*(.+)"),
      scope: vec![
        Scope {
            a: 46446339025403925,
            b: 8444249301319680,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628100252346,
            b: 5911103359942656,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172821,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 52636628100252346,
            b: 5911103359942656,
        },
    ]),(4, vec![
        Scope {
            a: 47288620757877339,
            b: 5911103359942656,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097877534,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }