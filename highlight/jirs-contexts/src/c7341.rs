
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
      regex: Regex::new("(?x:\\b  (?i:acosh|asinh|atanh|hypot|norm2|erf|erfc|erfc_scaled|gamma|log_gamma)  \\b  (?=\\s*\\()  )"),
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
      regex: Regex::new("(?x:\\b  (?i:bge|bgt|ble|blt|dshiftl|dshiftr|iall|iany|iparity|leadz|trailz|maskl|maskr|merge_bits|parity|popcnt|poppar)  \\b  (?=\\s*\\()  )"),
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
      regex: Regex::new("(?x:\\b  (?i:execute_command_line|storage_size|is_contiguous)  \\b  (?=\\s*\\()  )"),
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
      regex: Regex::new("(?x:\\b  (?i:int8|int16|int32|int64|real32|real64|real128)  \\b  )"),
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
      regex: Regex::new("(?x:\\b  (?i:character_kinds|integer_kinds|logical_kinds|real_kinds)  \\b  )"),
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
]
} }