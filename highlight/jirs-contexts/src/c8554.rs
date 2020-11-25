
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
      regex: Regex::new("(?<=^|[.,:;\\s])/[^/]+/(?=(?m:$)|[.,:;\\s])"),
      scope: vec![
        Scope {
            a: 114282585764462592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=^|[.,:;\\s])\\*[^*]+\\*(?=(?m:$)|[.,:;\\s])"),
      scope: vec![
        Scope {
            a: 114281679526363136,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=^|[.,:;\\s])_[^_]+_(?=(?m:$)|[.,:;\\s])"),
      scope: vec![
        Scope {
            a: 114280588604669952,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=^|[.,:;\\s])([~=]).+?\\1(?=(?m:$)|[.,:;\\s])"),
      scope: vec![
        Scope {
            a: 114280120494129281,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[(?:\\[([^\\]]*)\\])?\\[([^\\]]*)\\]\\]"),
      scope: vec![
        Scope {
            a: 46443487129829376,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114280588597985409,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451536781410433,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:http|ftp)s?:\\/\\/\\S+"),
      scope: vec![
        Scope {
            a: 114280588631933057,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }