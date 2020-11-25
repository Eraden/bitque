
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 34 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&[hH])\\h+(&)?(?=(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)|[=><]|(?:[+*^&/\\\\-]|\\b(?i:Mod)\\b)|\\b(?i:And|Not|Or|Xor|Is)\\b|[,)_])"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 1125899906842624,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 1125899906842624,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070552,
            b: 1125899906842624,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:\\d+\\.\\d*|\\.?\\d+)(?i:e[+-]?\\d+)?)(?=(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)|[=><]|(?:[+*^&/\\\\-]|\\b(?i:Mod)\\b)|\\b(?i:And|Not|Or|Xor|Is)\\b|[,)_])"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 1125899906842624,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:vbTrue|vbFalse|vbCr|vbCrLf|vbFormFeed|vbLf|vbNewLine|vbNullChar|vbNullString|vbTab|vbVerticalTab|vbBinaryCompare|vbTextCompare|vbSunday|vbMonday|vbTuesday|vbWednesday|vbThursday|vbFriday|vbSaturday|vbUseSystemDayOfWeek|vbFirstJan1|vbFirstFourDays|vbFirstFullWeek|vbGeneralDate|vbLongDate|vbShortDate|vbLongTime|vbShortTime|vbObjectError|vbEmpty|vbNull|vbInteger|vbLong|vbSingle|vbDouble|vbCurrency|vbDate|vbString|vbObject|vbError|vbBoolean|vbVariant|vbDataObject|vbDecimal|vbByte|vbArray|vbOkCancel|vbOkOnly|vbYesNo|vbYesNoCancel|vbAbortRetryIgnore|vbRetryCancel|vbYes|vbNo|vbAbort|vbCancel|vbIgnore|vbRetry|vbCritical|vbExclamation|vbInformation|vbQuestion|vbDefaultButton[123])\\b"),
      scope: vec![
        Scope {
            a: 59955110637469696,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Empty|False|Nothing|Null|True)\\b"),
      scope: vec![
        Scope {
            a: 61925375359123460,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 1125899906842624,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 68 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 79 })),
]
} }