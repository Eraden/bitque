
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
      regex: Regex::new("(?xi:\\b ieee_(class_type|round_type) \\b)"),
      scope: vec![
        Scope {
            a: 61925375491768320,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(signaling_nan|negative_normal|quiet_nan|positive_denormal|positive_inf|negative_denormal|negative_inf|positive_zero|positive_normal|negative_zero|other_value) \\b )"),
      scope: vec![
        Scope {
            a: 61925409851506688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(nearest|to_zero|up|other|down) \\b )"),
      scope: vec![
        Scope {
            a: 61925409851506688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(flag_type|status_type) \\b)"),
      scope: vec![
        Scope {
            a: 61925375491768320,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(invalid|divide_by_zero|overflow|inexact|underflow|usual|all) \\b )"),
      scope: vec![
        Scope {
            a: 61925409851506688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(features_type|status_type) \\b)"),
      scope: vec![
        Scope {
            a: 61925375491768320,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(datatype|inf|divide|nan|rounding|inexact_flag|sqrt|invalid_flag|denormal|underflow_flag|halting) \\b )"),
      scope: vec![
        Scope {
            a: 61925409851506688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(class|copy_sign) \\b )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b (call) \\s* (ieee_(get|set)_(flag|halting_mode|rounding_mode|status|underflow_mode)) \\b )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52645514533208064,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_is_(finite|nan|negative|normal) \\b )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(logb|next_after|rem|rint|scalb|selected_real_kind) \\b )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_support_(datatype|denormal|divide|flag|halting|inf|io|nan|rounding|sqrt|standard|underflow_control) \\b )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b ieee_(unordered|value) \\b )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }