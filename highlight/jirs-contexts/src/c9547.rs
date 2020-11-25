
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
      regex: Regex::new("(?!(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?(?:(\\bdeclare)\\s+)?\\b(const(?!\\s+enum\\b))(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.)))((?=;|}|((?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(of|in)\\s+)|^\\s*(?m:$)|;|(?:^\\s*(?:abstract|async|class|const|declare|enum|export|function|import|interface|let|module|namespace|return|type|var)\\b))|((?<!^const|[^\\._$\\p{L}\\p{N}]const)(?=\\s*(?m:$))))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?(?:(\\bdeclare)\\s+)?\\b(const(?!\\s+enum\\b))(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636728328342,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576472358912,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9548 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9583 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9696 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9699 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(,)\\s*((?!\\S)|(?=\\/\\/))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620757155990,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9549 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9653 })),
]
} }