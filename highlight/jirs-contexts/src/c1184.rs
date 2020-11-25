
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
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288620722159616,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1219 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?={)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1193 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=@|\\b(static|abstract|final|override|synchronized|scope|__gshared|nothrow|pure|ref|return|auto|extern|align|deprecated|pragma|private|protected|public|export|package)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(const|immutable|inout|shared)\\b(?!\\s*\\()"),
      scope: vec![
        Scope {
            a: 48414439024623616,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(else|enum|export|extern|debug|default|delegate|delete|deprecated|do|body|break|case|cast|catch|class|const|continue|abstract|alias|align|asm|assert|auto|final|finally|for|foreach|foreach_reverse|function|goto|if|immutable|import|in|inout|interface|invariant|is|lazy|macro|mixin|module|new|nothrow|out|override|package|pragma|private|protected|public|pure|ref|return|scope|shared|static|struct|switch|synchronized|template|throw|try|typeid|typeof|union|unittest|version|while|with|__gshared|__traits|__vector|__parameters|bool|byte|cdouble|cent|cfloat|char|creal|dchar|double|float|idouble|ifloat|int|ireal|long|real|short|ubyte|ucent|uint|ulong|ushort|void|wchar|string|dstring|wstring|null|true|false|__FILE__|__FILE_FULL_PATH__|__MODULE__|__LINE__|__FUNCTION__|__PRETTY_FUNCTION__|__DATE__|__EOF__|__TIME__|__TIMESTAMP__|__VENDOR__|__VERSION__|__ctfe|this|super)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=~?\\s*this\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1212 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b[\\p{L}_][\\p{L}0-9_]*\\b\\s*[,=\\()])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1256 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1256 }),
        ContextReference::Direct(ContextId { index: 1335 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1303 })),
]
} }