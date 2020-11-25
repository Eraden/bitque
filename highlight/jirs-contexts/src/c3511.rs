
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
      regex: Regex::new("(?:\\[\\s*(\\])|\\((\\))|\\(\\s*(\\)))"),
      scope: vec![
        Scope {
            a: 59955110706479156,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46447915236065280,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46447915243798580,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46447915236065280,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(true|false)\\b"),
      scope: vec![
        Scope {
            a: 59955110657196084,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'(.|\\\\(x[a-fA-F0-9][a-fA-F0-9]|[0-2]\\d\\d|[bnrt\'\"\\\\]))\'"),
      scope: vec![
        Scope {
            a: 59955200834928640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:-|\\b))(0[xX])(_*)(?:\\h[\\h_]*)(?:(\\.)[\\h_]*(?:[Pp][-+]??(_?)(?:[0-9][0-9_]*))?|(?:[Pp][-+]??(_?)(?:[0-9][0-9_]*)))"),
      scope: vec![
        Scope {
            a: 59955089176592600,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070764,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397940,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:-|\\b))(?:[0-9][0-9_]*)(?:(\\.)[0-9_]*(?:[Ee][-+]??(_?)(?:[0-9][0-9_]*))?|(?:[Ee][-+]??(_?)(?:[0-9][0-9_]*)))"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397940,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:-|\\b))(0[xX])(_*)(?:\\h[\\h_]*)*((?:[lLn]|(?!\\.))\\b)"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070764,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553268,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:-|\\b))(0[oO])(_*)(?:[0-7][0-7_]*)*((?:[lLn]|(?!\\.))\\b)"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070764,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553268,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:-|\\b))(0[bB])(_*)(?:[01][01_]*)*((?:[lLn]|(?!\\.))\\b)"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070764,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553268,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:-|\\b))(?:[0-9][0-9_]*)((?:[lLn]|(?!\\.))\\b)"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553268,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:-|\\b)\\d+.*\\b"),
      scope: vec![
        Scope {
            a: 50103314667667508,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }