
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
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 50103314668584964,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
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
      operation: MatchOperation::None,
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
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:vbTrue|vbFalse|vbCr|vbCrLf|vbFormFeed|vbLf|vbNewLine|vbNullChar|vbNullString|vbTab|vbVerticalTab|vbBinaryCompare|vbTextCompare|vbSunday|vbMonday|vbTuesday|vbWednesday|vbThursday|vbFriday|vbSaturday|vbUseSystemDayOfWeek|vbFirstJan1|vbFirstFourDays|vbFirstFullWeek|vbGeneralDate|vbLongDate|vbShortDate|vbLongTime|vbShortTime|vbObjectError|vbEmpty|vbNull|vbInteger|vbLong|vbSingle|vbDouble|vbCurrency|vbDate|vbString|vbObject|vbError|vbBoolean|vbVariant|vbDataObject|vbDecimal|vbByte|vbArray|vbOkCancel|vbOkOnly|vbYesNo|vbYesNoCancel|vbAbortRetryIgnore|vbRetryCancel|vbYes|vbNo|vbAbort|vbCancel|vbIgnore|vbRetry|vbCritical|vbExclamation|vbInformation|vbQuestion|vbDefaultButton[123])\\b"),
      scope: vec![
        Scope {
            a: 61925375359123460,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Abs|Array|Add|Asc|Atn|CBool|CByte|CCur|CDate|CDbl|Chr|CInt|CLng|Conversions|Cos|CreateObject|CSng|CStr|Date|DateAdd|DateDiff|DatePart|DateSerial|DateValue|Day|Derived|Math|Escape|Eval|Exists|Exp|Filter|FormatCurrency|FormatDateTime|FormatNumber|FormatPercent|GetLocale|GetObject|GetRef|Hex|Hour|InputBox|InStr|InStrRev|Int|Fix|IsArray|IsDate|IsEmpty|IsNull|IsNumeric|IsObject|Item|Items|Join|Keys|LBound|LCase|Left|Len|LoadPicture|Log|LTrim|RTrim|Trim|Maths|Mid|Minute|Month|MonthName|MsgBox|Now|Oct|Remove|RemoveAll|Replace|RGB|Right|Rnd|Round|ScriptEngine|ScriptEngineBuildVersion|ScriptEngineMajorVersion|ScriptEngineMinorVersion|Second|SetLocale|Sgn|Sin|Space|Split|Sqr|StrComp|String|StrReverse|Tan|Time|TimeSerial|TimeValue|TypeName|UBound|UCase|Unescape|VarType|Weekday|WeekdayName|Year)\\b"),
      scope: vec![
        Scope {
            a: 61925255100039172,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Empty|False|Nothing|Null|True)\\b"),
      scope: vec![
        Scope {
            a: 48414576462790656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 75 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:New)(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![
        Scope {
            a: 52636787027542020,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 49 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Class|Sub|Function|Const|Dim|ReDim|Public|Private|End|Preserve|Select|Case|If|Else|ElseIf|Then|For|Each|Next|ByRef|ByVal|Set|Call|New|Option|With|To|In|While|Wend|Until|Loop|On|GoTo|Resume|Let|Get|Exit|Do)\\b"),
      scope: vec![
        Scope {
            a: 50103314665439462,
            b: 1125899906842624,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
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
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 68 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:Request))\\b(?:(?i:(\\.)\\s*(?:(BinaryRead)|(ClientCertificate|Cookies|Form|QueryString|ServerVariables)|(TotalBytes)))\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366754967552,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226932740,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255085817856,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925366769909764,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925495603986432,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:Response))\\b(?:(?i:(\\.)\\s*(?:(AddHeader|AppendToLog|BinaryWrite|Clear|End|Flush|Redirect|Write)|(Buffer|CacheControl|Charset|CodePage|ContentType|Expires|ExpiresAbsolute|IsClientConnected|LCID|PICS|Status)|(Cookies)))\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366754967552,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226932740,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255085817856,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925495603986432,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925366769909764,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:Server))\\b(?:(?i:(\\.)\\s*(?:(CreateObject|Execute|GetLastError|HTMLEncode|MapPath|Transfer|URLEncode)|(ScriptTimeout)))\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366754967552,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226932740,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255085817856,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925495603986432,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:Server))\\b(?:(?i:(\\.)\\s*(?:(Abandon)|(CodePage|LCID|SessionID|Timeout)|(Contents|StaticObjects)))\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366754967552,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226932740,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255085817856,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925495603986432,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925366769909764,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:ObjectContext))\\b(?:(?i:(\\.)\\s*(SetAbort|SetComplete))\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366754967552,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226932740,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255085817856,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:Application))\\b(?:(?i:(\\.)\\s*(?:(Lock|Unlock)|(Contents|StaticObjects)))\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366754967552,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226932740,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255085817856,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925366769909764,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-zA-Z]\\w*|\\[(?:(?!%>|\\]).)*(?:\\]|(\\n|(?=%>)))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087292268544,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 12 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521948004534,
            b: 1125899906842624,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 67 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 34 })),
]
} }