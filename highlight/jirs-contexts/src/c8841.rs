
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
        a: 845021930586112,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845021930586112,
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
      regex: Regex::new("\\$\\{(\\d+|\\d+\\.\\d*|0[bB][01]+|0[oO][0-7]+|0[xX][0-9a-fA-F]+)\\}"),
      scope: vec![
        Scope {
            a: 59955089171480576,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(%\\{[\\w|\\s]+\\})"),
      scope: vec![
        Scope {
            a: 49258876847718400,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?<!\\\\)|(?<=\\\\\\\\))[$&]{"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8834 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?<!\\\\)(@{)|(?<=\\\\\\\\)(@{))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8835 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)^(\\|\\s*)?(\\*{1,3} ?)(settings?|variables?|keywords?|test cases?)( ?\\*{1,3})?(\\s*\\|)?"),
      scope: vec![
        Scope {
            a: 55451167450202112,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(^\\|\\s*)?\\[?Documentation\\]?\\s+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8836 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^| {2,}|\\t|\\\\| {1,})(?<!\\\\\\\\)#"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8837 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\s+\\[(Tags|Setup|Teardown|Template|Timeout|Arguments|Return)\\]"),
      scope: vec![
        Scope {
            a: 48414576462528512,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)^(\\|\\s*)?(Library|Resource|Test Timeout|Test Template|Test Teardown|Test Setup|Default Tags|Force Tags|Metadata|Variables|Suite Setup|Suite Teardown)(?:(  )|( \\| ))"),
      scope: vec![
        Scope {
            a: 59955110637207552,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\|\\s+)?(?!^\\.{3})(?![\\|$&])\\S+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8838 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)^\\s+(Given|When|and|but|Then)"),
      scope: vec![
        Scope {
            a: 55451167574982656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }