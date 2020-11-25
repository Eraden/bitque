
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
        a: 844613912494080,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844613912494080,
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
      regex: Regex::new("\\(\\?\\#"),
      scope: vec![
        Scope {
            a: 47288629323038902,
            b: 12384898975268864,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4166 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\{1,2}[1-9]"),
      scope: vec![
        Scope {
            a: 52636787067453484,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\{1,2}g([1-9]|\\{-?[0-9]+\\})"),
      scope: vec![
        Scope {
            a: 52636787067453484,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\{1,2}(?:[gk]\\{([^\\}]+)\\}|k(?:<([^>]+)>|\'([^\']+)\'))"),
      scope: vec![
        Scope {
            a: 52636787067454201,
            b: 12384898975268864,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(\\?P=)([a-zA-Z_][a-zA-Z_0-9]*\\w*)(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 52636787067454201,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ]),(4, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\{1,2}[bBAZzG]|\\^|\\$"),
      scope: vec![
        Scope {
            a: 52636636743532588,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[?+*][?+]?|\\{([0-9]+(,([0-9]+)?)?|,[0-9]+)\\}\\??"),
      scope: vec![
        Scope {
            a: 52636628154253356,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|"),
      scope: vec![
        Scope {
            a: 52636628154187820,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\?R\\)"),
      scope: vec![
        Scope {
            a: 52636787093405740,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\?[imsxJUX]+\\)"),
      scope: vec![
        Scope {
            a: 52636787093471276,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()((\\?=)|(\\?!)|(\\?<=)|(\\?<!)|(\\?>)|(\\?:)|(\\?\\|)|(\\?[imsxJUX]+:))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 59955136454393900,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46446476473729068,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46446476473794604,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 46446476473860140,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 46446476473925676,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 46446476499615788,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 46446476499681324,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 46446476499746860,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 52636787093471276,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4167 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(\\?)(?:(P)?(<)([^>]+)(>)|(\')([^\']+)(\'))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 59955136454393900,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955136454393900,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629318583511,
            b: 51228634739900416,
        },
    ]),(5, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ]),(6, vec![
        Scope {
            a: 47288629318583511,
            b: 48132409996083200,
        },
    ]),(7, vec![
        Scope {
            a: 47288629318583511,
            b: 51228634739900416,
        },
    ]),(8, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ]),(9, vec![
        Scope {
            a: 47288629318583511,
            b: 48132409996083200,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4168 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()"),
      scope: vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4169 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4172 })),
]
} }