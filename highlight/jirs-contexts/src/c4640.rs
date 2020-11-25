
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
        a: 844613912756224,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844613912756224,
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
      regex: Regex::new("(#)[^]\\[(){}^$+*?\\\\|\"\']*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711032873004,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038764,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[bBAZzG]|\\^|\\$"),
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
      regex: Regex::new("\\\\[1-9][0-9]?"),
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
      regex: Regex::new("[?+*][?+]?|\\{(\\d+,\\d+|\\d+,|,\\d+|\\d+)\\}\\??"),
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
      regex: Regex::new("\\(\\?\\#"),
      scope: vec![
        Scope {
            a: 47288629323038902,
            b: 12384898975268864,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4633 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\?[iLmsux]+\\)"),
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
      regex: Regex::new("(\\()(\\?P=([a-zA-Z_][a-zA-Z_0-9]*\\w*))(\\))"),
      scope: vec![
        Scope {
            a: 52636787067454201,
            b: 12384898975268864,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()((\\?=)|(\\?!)|(\\?<=)|(\\?<!))"),
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
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4634 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(\\?\\(([1-9][0-9]?|[a-zA-Z_][a-zA-Z_0-9]*)\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 47288629318582995,
            b: 98516430827290624,
        },
    ]),(3, vec![
        Scope {
            a: 49259087346925612,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4635 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()((\\?P<)([a-z]\\w*)(>)|(\\?:))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318583511,
            b: 12384898975268864,
        },
    ]),(4, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318583511,
            b: 12384898975268864,
        },
    ]),(6, vec![
        Scope {
            a: 47288629318583117,
            b: 12384898975268864,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4636 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4639 })),
]
} }