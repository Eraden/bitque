
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3708 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*|&"),
      scope: vec![
        Scope {
            a: 52636628102414336,
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
            a: 52636636692348928,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444084125499392,
            b: 0,
        },
        Scope {
            a: 59392130630615096,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3762 }),
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
        ContextReference::Direct(ContextId { index: 3720 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (struct|union|enum\\s+class|enum\\s+struct|enum|class)\n  \\s+\n  (?=\n    (?![\\p{Lu}\\p{Nd}_]+\\b|__declspec|struct|union|enum\\s+class|enum\\s+struct|enum|class)\n    (?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\n    (\\s+\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*\\(|\\s*[*&])\n  )\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466198528,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3593 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(struct|union|enum\\s+class|enum\\s+struct|enum|class)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3710 }),
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3703 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3812 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3816 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3709 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 604 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[&*])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3720 }),
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