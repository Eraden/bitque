
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
      regex: Regex::new("\\\\[?*+]"),
      scope: vec![
        Scope {
            a: 59955200847315791,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\\\."),
      scope: vec![
        Scope {
            a: 59955200847314976,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\x\\h\\h|(\\\\x(?!{))|(?>\\\\x(?>{\\h+}|{(?>\\h|([^}]))+}|({}?)|))"),
      scope: vec![
        Scope {
            a: 59955200847315536,
            b: 12385324177031168,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314669437520,
            b: 12385324177031168,
        },
    ]),(2, vec![
        Scope {
            a: 50103314669437520,
            b: 12385324177031168,
        },
    ]),(3, vec![
        Scope {
            a: 50103314669437520,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\0[0-7]{1,2}"),
      scope: vec![
        Scope {
            a: 59955200847315298,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\o(?!{))|(?>\\\\o(?>{[0-7]+}|{(?>[0-7]|([^}]))+}|({}?)|))"),
      scope: vec![
        Scope {
            a: 59955200847315298,
            b: 12385324177031168,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314669437282,
            b: 12385324177031168,
        },
    ]),(2, vec![
        Scope {
            a: 50103314669437282,
            b: 12385324177031168,
        },
    ]),(3, vec![
        Scope {
            a: 50103314669437282,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\c"),
      scope: vec![
        Scope {
            a: 59955200847317116,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6895 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[aefnrt]"),
      scope: vec![
        Scope {
            a: 59955200847317134,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\."),
      scope: vec![
        Scope {
            a: 59955200847314988,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }