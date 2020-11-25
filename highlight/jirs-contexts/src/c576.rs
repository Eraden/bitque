
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 559 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*|&"),
      scope: vec![
        Scope {
            a: 52636628099530752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(operator)\\s+(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\s*(\\(|(?m:$)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636689465344,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444084122615808,
            b: 0,
        },
        Scope {
            a: 59392130630615052,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 617 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(break|case|catch|continue|default|do|else|for|goto|if|_Pragma|return|switch|throw|try|while|and|and_eq|bitand|bitor|compl|not|not_eq|or|or_eq|xor|xor_eq|noexcept|const_cast|dynamic_cast|reinterpret_cast|static_cast|new|delete|typedef|nullptr|private|protected|public|static_assert|sizeof|using|typeid|alignof|alignas|namespace|template|operator)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 571 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (struct|union|enum\\s+class|enum\\s+struct|enum|class)\n  \\s+\n  (?=\n    (?![\\p{Lu}\\p{Nd}_]+\\b|__declspec|struct|union|enum\\s+class|enum\\s+struct|enum|class)\n    (?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\n    (\\s+\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*\\(|\\s*[*&])\n  )\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576463314944,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 452 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(struct|union|enum\\s+class|enum\\s+struct|enum|class)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 561 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(const_cast|dynamic_cast|reinterpret_cast|static_cast)\\b\\s*<)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=<<|<=)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 555 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 663 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 674 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 560 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 604 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[&*])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 571 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\W)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }