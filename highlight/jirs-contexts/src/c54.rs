
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
      regex: Regex::new("\\b(?i:Empty|False|Nothing|Null|True)\\b|\\b(?i:vbTrue|vbFalse|vbCr|vbCrLf|vbFormFeed|vbLf|vbNewLine|vbNullChar|vbNullString|vbTab|vbVerticalTab|vbBinaryCompare|vbTextCompare|vbSunday|vbMonday|vbTuesday|vbWednesday|vbThursday|vbFriday|vbSaturday|vbUseSystemDayOfWeek|vbFirstJan1|vbFirstFourDays|vbFirstFullWeek|vbGeneralDate|vbLongDate|vbShortDate|vbLongTime|vbShortTime|vbObjectError|vbEmpty|vbNull|vbInteger|vbLong|vbSingle|vbDouble|vbCurrency|vbDate|vbString|vbObject|vbError|vbBoolean|vbVariant|vbDataObject|vbDecimal|vbByte|vbArray|vbOkCancel|vbOkOnly|vbYesNo|vbYesNoCancel|vbAbortRetryIgnore|vbRetryCancel|vbYes|vbNo|vbAbort|vbCancel|vbIgnore|vbRetry|vbCritical|vbExclamation|vbInformation|vbQuestion|vbDefaultButton[123])\\b|\\b(?i:Class|Sub|Function|Const|Dim|ReDim|Public|Private|End|Preserve|Select|Case|If|Else|ElseIf|Then|For|Each|Next|ByRef|ByVal|Set|Call|New|Option|With|To|In|While|Wend|Until|Loop|On|GoTo|Resume|Let|Get|Exit|Do)\\b|\\b(?i:Abs|Array|Add|Asc|Atn|CBool|CByte|CCur|CDate|CDbl|Chr|CInt|CLng|Conversions|Cos|CreateObject|CSng|CStr|Date|DateAdd|DateDiff|DatePart|DateSerial|DateValue|Day|Derived|Math|Escape|Eval|Exists|Exp|Filter|FormatCurrency|FormatDateTime|FormatNumber|FormatPercent|GetLocale|GetObject|GetRef|Hex|Hour|InputBox|InStr|InStrRev|Int|Fix|IsArray|IsDate|IsEmpty|IsNull|IsNumeric|IsObject|Item|Items|Join|Keys|LBound|LCase|Left|Len|LoadPicture|Log|LTrim|RTrim|Trim|Maths|Mid|Minute|Month|MonthName|MsgBox|Now|Oct|Remove|RemoveAll|Replace|RGB|Right|Rnd|Round|ScriptEngine|ScriptEngineBuildVersion|ScriptEngineMajorVersion|ScriptEngineMinorVersion|Second|SetLocale|Sgn|Sin|Space|Split|Sqr|StrComp|String|StrReverse|Tan|Time|TimeSerial|TimeValue|TypeName|UBound|UCase|Unescape|VarType|Weekday|WeekdayName|Year)\\b|\\b(?i:Application|ObjectContext|Request|Response|Server|Session)\\b"),
      scope: vec![
        Scope {
            a: 50103314667536388,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }