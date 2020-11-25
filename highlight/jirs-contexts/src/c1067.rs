
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1054 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("defprotocol[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 48414576463511552,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1059 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("definterface[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 48414576463511552,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1057 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:deftype|defrecord)[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 48414576463511552,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1060 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("defmethod[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 48414439043497999,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1064 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("def[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 48414439043497999,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1056 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:reify|proxy|extend-protocol|extend-type)[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 49258881134559232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1073 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("fn\\*?(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\])"),
      scope: vec![
        Scope {
            a: 48414439059685391,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1064 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("declare[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 48414439059750927,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1058 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1066 }),
    ]),
      with_prototype: None
    }),
]
} }