
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
        index: 399,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(using)\\s+(?=(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*=\\s*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449227,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 425 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(using)(?!\\s*\\()(?:\\s+(static))?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449227,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636717449227,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 425 }),
    ]),
      with_prototype: None
    }),
]
} }