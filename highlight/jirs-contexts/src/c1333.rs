
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(else|enum|export|extern|debug|default|delegate|delete|deprecated|do|body|break|case|cast|catch|class|const|continue|abstract|alias|align|asm|assert|auto|final|finally|for|foreach|foreach_reverse|function|goto|if|immutable|import|in|inout|interface|invariant|is|lazy|macro|mixin|module|new|nothrow|out|override|package|pragma|private|protected|public|pure|ref|return|scope|shared|static|struct|switch|synchronized|template|throw|try|typeid|typeof|union|unittest|version|while|with|__gshared|__traits|__vector|__parameters)\\b"),
      scope: vec![
        Scope {
            a: 52635889364369408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(bool|byte|cdouble|cent|cfloat|char|creal|dchar|double|float|idouble|ifloat|int|ireal|long|real|short|ubyte|ucent|uint|ulong|ushort|void|wchar|string|dstring|wstring)\\b"),
      scope: vec![
        Scope {
            a: 48414576463577088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(null|true|false|__FILE__|__FILE_FULL_PATH__|__MODULE__|__LINE__|__FUNCTION__|__PRETTY_FUNCTION__|__DATE__|__EOF__|__TIME__|__TIMESTAMP__|__VENDOR__|__VERSION__|__ctfe)\\b"),
      scope: vec![
        Scope {
            a: 59955110638256128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(this|super)\\b"),
      scope: vec![
        Scope {
            a: 49259061523251200,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1200 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1321 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1305 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\p{L}_][\\p{L}0-9_]*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(/|&|\\||-|\\+|<<|<>|>>|>>>||\\*|%|\\^|\\^\\^|~)="),
      scope: vec![
        Scope {
            a: 52636628111130640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(/|<=|>=|==|!<>=|!<=|!>=|\\.\\.\\.|\\.\\.|&|&&|\\||\\|\\||-|--|\\+|\\+\\+|<|<<|<>|>|>>|>>>|!|!<>|!<|!>|\\?|,|:|\\$|\\*|%|\\^|\\^\\^|~|@|=>|#)"),
      scope: vec![
        Scope {
            a: 52636628099792896,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288689441636352,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\."),
      scope: vec![
        Scope {
            a: 47288788226932752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521948004534,
            b: 4503599627370496,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1140 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521961570486,
            b: 4503599627370496,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1141 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288521962160310,
            b: 4503599627370496,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1142 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1302 })),
]
} }