
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
  prototype: Some(
    ContextId {
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\"\"\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 20547673299877888,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5180 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 20547673299877888,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(f)(\"\"\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255090339840,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 20547673299877888,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5182 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:[$\\p{Lu}](?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?)|(?:(?:\\p{Ll}|_+(?=[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]))(?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?))))(\"\"\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255090339840,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 20547673299877888,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5183 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(f)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255090339840,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 20547673299877888,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5184 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(raw)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255090339840,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 20547673299877888,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5185 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:[$\\p{Lu}](?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?)|(?:(?:\\p{Ll}|_+(?=[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]))(?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?))))(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255090339840,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 20547673299877888,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5186 }),
    ]),
      with_prototype: None
    }),
]
} }