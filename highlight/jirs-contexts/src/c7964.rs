
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
      regex: Regex::new("(?x)\n(\\b|(?=.))\n(?: # Dashes betwen numeric symbols (11 = 1_1) are allowed everywhere.\n  0b[0-1](?:_?[0-1])*|             # binary\n  0o[0-7](?:_?[0-7])*|             # octal\n  0x[\\da-fA-F](?:_?[\\da-fA-F])*|   # hex\n  (?:\n    \\.\\d(?:_?\\d)*|                      # .11, .11\n    \\d(?:_?\\d)*(?:\\.(?:\\d(?:_?\\d)*)?)?  # 11.11, 11., 11\n  )\n  (?:e[-+]?\\d(?:_?\\d)*)?                # Any of the above followed by e+123 or similar, for scientific notation.\n)"),
      scope: vec![
        Scope {
            a: 59955089170300928,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7958 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(true|false|nothing|missing|ℯ|pi|π|im|undef|NaN|NaN16|NaN32|NaN64|Inf|Inf16|Inf32|Inf64|ARGS|C_NULL|ENDIAN_BOM|ENV|LOAD_PATH|PROGRAM_FILE|STDERR|STDIN|STDOUT|VERSION)\\b"),
      scope: vec![
        Scope {
            a: 59955110645137408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7958 }),
    ]),
      with_prototype: None
    }),
]
} }