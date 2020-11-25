
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "ASP".to_string(),
  file_extensions: vec!["asa".to_string()],
  scope: Scope { a: 844442110001152, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("reserved_words".to_string(), "\\b(?i:Class|Sub|Function|Const|Dim|ReDim|Public|Private|End|Preserve|Select|Case|If|Else|ElseIf|Then|For|Each|Next|ByRef|ByVal|Set|Call|New|Option|With|To|In|While|Wend|Until|Loop|On|GoTo|Resume|Let|Get|Exit|Do)\\b".to_string());
    v.insert("logical_operators".to_string(), "\\b(?i:And|Not|Or|Xor|Is)\\b".to_string());
    v.insert("apostrophe_comment_begin".to_string(), "\'".to_string());
    v.insert("math_operators".to_string(), "(?:[+*^&/\\\\-]|\\b(?i:Mod)\\b)".to_string());
    v.insert("operators".to_string(), "{{comparison_operators}}|{{math_operators}}|{{logical_operators}}".to_string());
    v.insert("literal_number_decimal".to_string(), "(?:(?:\\d+\\.\\d*|\\.?\\d+)(?i:e[+-]?\\d+)?)(?={{whitespace_or_end_of_statement}}|{{operators}}|[,)_])".to_string());
    v.insert("functions".to_string(), "\\b(?i:Abs|Array|Add|Asc|Atn|CBool|CByte|CCur|CDate|CDbl|Chr|CInt|CLng|Conversions|Cos|CreateObject|CSng|CStr|Date|DateAdd|DateDiff|DatePart|DateSerial|DateValue|Day|Derived|Math|Escape|Eval|Exists|Exp|Filter|FormatCurrency|FormatDateTime|FormatNumber|FormatPercent|GetLocale|GetObject|GetRef|Hex|Hour|InputBox|InStr|InStrRev|Int|Fix|IsArray|IsDate|IsEmpty|IsNull|IsNumeric|IsObject|Item|Items|Join|Keys|LBound|LCase|Left|Len|LoadPicture|Log|LTrim|RTrim|Trim|Maths|Mid|Minute|Month|MonthName|MsgBox|Now|Oct|Remove|RemoveAll|Replace|RGB|Right|Rnd|Round|ScriptEngine|ScriptEngineBuildVersion|ScriptEngineMajorVersion|ScriptEngineMinorVersion|Second|SetLocale|Sgn|Sin|Space|Split|Sqr|StrComp|String|StrReverse|Tan|Time|TimeSerial|TimeValue|TypeName|UBound|UCase|Unescape|VarType|Weekday|WeekdayName|Year)\\b".to_string());
    v.insert("literal_number_hex".to_string(), "(&[hH])\\h+(&)?(?={{whitespace_or_end_of_statement}}|{{operators}}|[,)_])".to_string());
    v.insert("whitespace_or_end_of_statement".to_string(), "(?=\\s|$|:|{{apostrophe_comment_begin}}|{{rem_comment_begin}}|%>)".to_string());
    v.insert("rem_comment_begin".to_string(), "\\b(?i:REM)\\b".to_string());
    v.insert("asp_builtin_classes".to_string(), "\\b(?i:Application|ObjectContext|Request|Response|Server|Session)\\b".to_string());
    v.insert("class_magic_funcs".to_string(), "\\b(?i:Class_Initialize|Class_Terminate)\\b".to_string());
    v.insert("keywords".to_string(), "\\b(?i:Empty|False|Nothing|Null|True)\\b".to_string());
    v.insert("asp_builtin_events".to_string(), "\\b(?i:Application_OnEnd|Application_OnStart|OnTransactionAbort|OnTransactionCommit|Session_OnEnd|Session_OnStart)\\b".to_string());
    v.insert("comparison_operators".to_string(), "[=><]".to_string());
    v.insert("constants".to_string(), "\\b(?i:vbTrue|vbFalse|vbCr|vbCrLf|vbFormFeed|vbLf|vbNewLine|vbNullChar|vbNullString|vbTab|vbVerticalTab|vbBinaryCompare|vbTextCompare|vbSunday|vbMonday|vbTuesday|vbWednesday|vbThursday|vbFriday|vbSaturday|vbUseSystemDayOfWeek|vbFirstJan1|vbFirstFourDays|vbFirstFullWeek|vbGeneralDate|vbLongDate|vbShortDate|vbLongTime|vbShortTime|vbObjectError|vbEmpty|vbNull|vbInteger|vbLong|vbSingle|vbDouble|vbCurrency|vbDate|vbString|vbObject|vbError|vbBoolean|vbVariant|vbDataObject|vbDecimal|vbByte|vbArray|vbOkCancel|vbOkOnly|vbYesNo|vbYesNoCancel|vbAbortRetryIgnore|vbRetryCancel|vbYes|vbNo|vbAbort|vbCancel|vbIgnore|vbRetry|vbCritical|vbExclamation|vbInformation|vbQuestion|vbDefaultButton[123])\\b".to_string());
    v.insert("identifier".to_string(), "[a-zA-Z]\\w*|\\[(?:(?!%>|\\]).)*(?:\\]|(\\n|(?=%>)))".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("inside_string".to_string(), ContextId { index: 68 });
    v.insert("#anon_inside_block_4".to_string(), ContextId { index: 17 });
    v.insert("#anon_inside_control_flow_do_0".to_string(), ContextId { index: 20 });
    v.insert("#anon_line_continuation_char_1".to_string(), ContextId { index: 26 });
    v.insert("inside_block".to_string(), ContextId { index: 55 });
    v.insert("allow_statement_separator".to_string(), ContextId { index: 35 });
    v.insert("inside_control_flow_do".to_string(), ContextId { index: 57 });
    v.insert("inside_control_flow_for".to_string(), ContextId { index: 58 });
    v.insert("expect_identifier_member_reference".to_string(), ContextId { index: 48 });
    v.insert("unexpected_token".to_string(), ContextId { index: 79 });
    v.insert("#anon_control_flow_0".to_string(), ContextId { index: 9 });
    v.insert("#anon_inside_class_1".to_string(), ContextId { index: 19 });
    v.insert("#anon_inside_block_1".to_string(), ContextId { index: 14 });
    v.insert("elseif_then_block".to_string(), ContextId { index: 47 });
    v.insert("root_asp".to_string(), ContextId { index: 76 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 6 });
    v.insert("#anon_inside_method_0".to_string(), ContextId { index: 24 });
    v.insert("for_after_to".to_string(), ContextId { index: 53 });
    v.insert("inside_control_flow_while".to_string(), ContextId { index: 63 });
    v.insert("after_method_name".to_string(), ContextId { index: 32 });
    v.insert("expect_not_end_of_statement".to_string(), ContextId { index: 50 });
    v.insert("illegal_names".to_string(), ContextId { index: 54 });
    v.insert("inside_control_flow_if_single_line".to_string(), ContextId { index: 61 });
    v.insert("dim_body".to_string(), ContextId { index: 46 });
    v.insert("constant_value".to_string(), ContextId { index: 41 });
    v.insert("#anon_expression_0".to_string(), ContextId { index: 12 });
    v.insert("definitions".to_string(), ContextId { index: 45 });
    v.insert("not_end_of_statement".to_string(), ContextId { index: 74 });
    v.insert("class_definitions".to_string(), ContextId { index: 36 });
    v.insert("#anon_inside_control_flow_select_0".to_string(), ContextId { index: 22 });
    v.insert("control_flow".to_string(), ContextId { index: 42 });
    v.insert("after_method_param_name".to_string(), ContextId { index: 33 });
    v.insert("control_flow_forto".to_string(), ContextId { index: 44 });
    v.insert("#anon_inside_control_flow_with_0".to_string(), ContextId { index: 23 });
    v.insert("#anon_line_continuation_char_0".to_string(), ContextId { index: 25 });
    v.insert("__main".to_string(), ContextId { index: 30 });
    v.insert("constant_separator".to_string(), ContextId { index: 40 });
    v.insert("inside_method".to_string(), ContextId { index: 65 });
    v.insert("method_name".to_string(), ContextId { index: 72 });
    v.insert("variable_definitions".to_string(), ContextId { index: 80 });
    v.insert("line_continuation_char".to_string(), ContextId { index: 69 });
    v.insert("inside_method_without_meta".to_string(), ContextId { index: 66 });
    v.insert("#anon_after_method_name_1".to_string(), ContextId { index: 4 });
    v.insert("inside_control_flow_with".to_string(), ContextId { index: 64 });
    v.insert("comments".to_string(), ContextId { index: 38 });
    v.insert("inside_class".to_string(), ContextId { index: 56 });
    v.insert("#anon_after_method_name_0".to_string(), ContextId { index: 3 });
    v.insert("#anon_constant_body_0".to_string(), ContextId { index: 8 });
    v.insert("#anon_inside_class_0".to_string(), ContextId { index: 18 });
    v.insert("#anon_not_end_of_statement_0".to_string(), ContextId { index: 27 });
    v.insert("control_flow_foreach_in".to_string(), ContextId { index: 43 });
    v.insert("operators".to_string(), ContextId { index: 75 });
    v.insert("expression".to_string(), ContextId { index: 51 });
    v.insert("#anon_statements_0".to_string(), ContextId { index: 28 });
    v.insert("method_name_without_meta".to_string(), ContextId { index: 73 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 7 });
    v.insert("inside_control_flow_if_common".to_string(), ContextId { index: 60 });
    v.insert("inside_control_flow_if_block".to_string(), ContextId { index: 59 });
    v.insert("#anon_inside_block_2".to_string(), ContextId { index: 15 });
    v.insert("main".to_string(), ContextId { index: 70 });
    v.insert("statements".to_string(), ContextId { index: 77 });
    v.insert("#anon_dim_body_1".to_string(), ContextId { index: 11 });
    v.insert("__start".to_string(), ContextId { index: 31 });
    v.insert("#anon_inside_block_0".to_string(), ContextId { index: 13 });
    v.insert("inside_control_flow_select".to_string(), ContextId { index: 62 });
    v.insert("#anon_inside_control_flow_if_block_0".to_string(), ContextId { index: 21 });
    v.insert("class_name".to_string(), ContextId { index: 37 });
    v.insert("expect_identifier_reference".to_string(), ContextId { index: 49 });
    v.insert("allow_line_continuation".to_string(), ContextId { index: 34 });
    v.insert("expression_until_end_of_statement".to_string(), ContextId { index: 52 });
    v.insert("then_could_be_block_or_single_line".to_string(), ContextId { index: 78 });
    v.insert("#anon_inside_block_3".to_string(), ContextId { index: 16 });
    v.insert("#anon_then_could_be_block_or_single_line_0".to_string(), ContextId { index: 29 });
    v.insert("constant_body".to_string(), ContextId { index: 39 });
    v.insert("#anon_after_method_name_2".to_string(), ContextId { index: 5 });
    v.insert("inside_parens".to_string(), ContextId { index: 67 });
    v.insert("method_definitions".to_string(), ContextId { index: 71 });
    v.insert("#anon_dim_body_0".to_string(), ContextId { index: 10 });
    v
  }
} }