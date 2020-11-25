
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
      regex: Regex::new("\\?<?[=!]"),
      scope: vec![
        Scope {
            a: 52636636736061484,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) \\? (?>P?<([a-zA-Z_][a-zA-Z_\\d]{,31})> | \'\\g<-1>\' | (P)(?=\') | P?(<>|\'\') | P?<([^\\[\\\\(){}|^$.?*+\\n]+?)> | P?\'\\g<-1>\')"),
      scope: vec![
        Scope {
            a: 52636787100745772,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130707292204,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 50103314796052524,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 50103314796052524,
            b: 27866022694354944,
        },
    ]),(4, vec![
        Scope {
            a: 50103314796052524,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?>"),
      scope: vec![
        Scope {
            a: 52636636769681452,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?:"),
      scope: vec![
        Scope {
            a: 52636636831154220,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?\\|"),
      scope: vec![
        Scope {
            a: 52636636831219756,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (\\?) ([imsxJUX]+(?:-[imsxJUX]*)? | (?:-[imsxJUX]*)+) (.*?) (:|(?=\\)))"),
      scope: vec![
        Scope {
            a: 48414439111721004,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439111723136,
            b: 12385324177031168,
        },
    ]),(2, vec![
        Scope {
            a: 48414439111723137,
            b: 12385324177031168,
        },
    ]),(3, vec![
        Scope {
            a: 50103314796380204,
            b: 27866022694354944,
        },
    ]),(4, vec![
        Scope {
            a: 48414439111721302,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\?\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6915 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
]
} }