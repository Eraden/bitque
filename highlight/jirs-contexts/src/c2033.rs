
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
      regex: Regex::new("\\$(?:(?:(?xi)\n  # Valid unicode letters according to:\n  # http://groovy-lang.org/syntax.html#_normal_identifiers\n  #   Literal Unicode         Escaped Unicode\n      [\\x{00C0}-\\x{00D6}]  |  \\\\u00C[0-9A-F] | \\\\u00D[0-6]\n    | [\\x{00D8}-\\x{00F6}]  |  \\\\u00D[89A-F]  | \\\\u00E[0-9A-F] | \\\\u00F[0-6]\n    | [\\x{00F8}-\\x{00FF}]  |  \\\\u00F[89A-F]\n    | [\\x{0100}-\\x{FFFE}]  |  \\\\u0[1-9A-F][0-9A-F]{2} | \\\\u(?!FFFF)[1-9A-F][0-9A-F]{3}\n)|[a-zA-Z_])(?:(?:(?xi)\n  # Valid unicode letters according to:\n  # http://groovy-lang.org/syntax.html#_normal_identifiers\n  #   Literal Unicode         Escaped Unicode\n      [\\x{00C0}-\\x{00D6}]  |  \\\\u00C[0-9A-F] | \\\\u00D[0-6]\n    | [\\x{00D8}-\\x{00F6}]  |  \\\\u00D[89A-F]  | \\\\u00E[0-9A-F] | \\\\u00F[0-6]\n    | [\\x{00F8}-\\x{00FF}]  |  \\\\u00F[89A-F]\n    | [\\x{0100}-\\x{FFFE}]  |  \\\\u0[1-9A-F][0-9A-F]{2} | \\\\u(?!FFFF)[1-9A-F][0-9A-F]{3}\n)|[a-zA-Z0-9_])*"),
      scope: vec![
        Scope {
            a: 49259087316254753,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2000 }),
    ]),
      with_prototype: None
    }),
]
} }