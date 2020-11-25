
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
      regex: Regex::new("(?=,|\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=:)\\s*(async)?(?=\\s*(<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*\\>)*>\\s*)\\(\\s*((([\\{\\[]\\s*)?(?m:$))|((\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})\\s*((:\\s*\\{?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))|((\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])\\s*((:\\s*\\[?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439108182166,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9489 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=:)\\s*(async)?\\s*(\\()(?=\\s*((([\\{\\[]\\s*)?(?m:$))|((\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})\\s*((:\\s*\\{?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))|((\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])\\s*((:\\s*\\[?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439108182166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445243847803030,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9480 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=:)\\s*(async)?\\s*(?=\\<\\s*(?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439108182166,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9481 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\>)\\s*(\\()(?=\\s*((([\\{\\[]\\s*)?(?m:$))|((\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})\\s*((:\\s*\\{?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))|((\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])\\s*((:\\s*\\[?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445243847803030,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9482 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9650 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9593 })),
]
} }