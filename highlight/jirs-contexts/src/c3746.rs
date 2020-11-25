
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
      regex: Regex::new("(?=\\b(break|case|catch|continue|default|do|else|for|goto|if|_Pragma|return|switch|throw|try|while|and|and_eq|bitand|bitor|compl|not|not_eq|or|or_eq|xor|xor_eq|noexcept|const_cast|dynamic_cast|reinterpret_cast|static_cast|new|delete|typedef|nullptr|private|protected|public|static_assert|sizeof|using|typeid|alignof|alignas|namespace|template)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(const|constexpr|mutable|typename|volatile)\\b"),
      scope: vec![
        Scope {
            a: 48414439027245056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3765 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3764 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\p{Lu}\\p{Nd}_]+\\s*(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=((?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(?:<(?:[^(){}&;*^%=<>-]+(?:<(?:[^(){}&;*^%=<>-]+(?:<[^(){}&;*^%=<>-]*>)?)?\\s*>)?)?[^(){}&;*^%=<>-]*(?:\\([^(){}&;*^%=<>-]*\\))?[^(){}&;*^%=<>-]*>)?::\\s*)?\\boperator\\s*(?:[-+*/%ˆ&|~!=<>]|[-+*/%^&|=!<>]=|<<=?|>>=?|&&|\\|\\||\\+\\+|--|,|->\\*?|\\(\\)|\\[\\]|\"\"\\s*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\(|(?m:$)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3612 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b([ \\t]+|[*&])(?!\\s*(<|::|\\(|(?m:$))))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3613 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(<(?:[^(){}&;*^%=<>-]+(?:<(?:[^(){}&;*^%=<>-]+(?:<[^(){}&;*^%=<>-]*>)?)?\\s*>)?)?[^(){}&;*^%=<>-]*(?:\\([^(){}&;*^%=<>-]*\\))?[^(){}&;*^%=<>-]*>((?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)?)\\s*(\\(|(?m:$)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3738 }),
        ContextReference::Direct(ContextId { index: 3745 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(\\(|(?m:$)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3738 }),
        ContextReference::Direct(ContextId { index: 3744 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3738 }),
        ContextReference::Direct(ContextId { index: 3744 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }