
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.(?:start|imagebase|tlvp|got(?:pc(?:rel)?|(?:tp)?off)?|plt|sym|tlsie)\\b"),
      scope: vec![
        Scope {
            a: 61925409709949015,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__(?:utf(?:(?:16|32)(?:[lb]e)?)|float(?:8|16|32|64|80[me]|128[lh])|Infinity|[QS]?NaN)__\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__NASM_(?:MAJOR|(?:SUB)?MINOR|SNAPSHOT|VER(?:SION_ID)?)__\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b___NASM_PATCHLEVEL__\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__(?:FILE|LINE|BITS|OUTPUT_FORMAT)__\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__(?:(?:UTC_)?(?:DATE|TIME)(?:_NUM)?|POSIX_TIME)__\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__USE_(?:ALTREG|SMARTALIGN|FP|IFUNC)__\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__PASS__\\b"),
      scope: vec![
        Scope {
            a: 50104723417333973,
            b: 479633725392879616,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\balignmode\\b"),
      scope: vec![
        Scope {
            a: 61925255190216789,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6055 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__ALIGNMODE__\\b"),
      scope: vec![
        Scope {
            a: 61925409809039445,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:Inf|[QS]?NaN)\\b"),
      scope: vec![
        Scope {
            a: 61925409816117333,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:float(?:8|16|32|64|80[me]|128[lh]))\\b"),
      scope: vec![
        Scope {
            a: 61925255197294677,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bilog2(?:[ewfc]|[fc]w)?\\b"),
      scope: vec![
        Scope {
            a: 61925255197360213,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }