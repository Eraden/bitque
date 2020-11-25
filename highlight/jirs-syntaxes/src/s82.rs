
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Tcl".to_string(),
  file_extensions: vec!["tcl".to_string()],
  scope: Scope { a: 844759937581056, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("unquoted_string".to_string(), "[^\\s{{special_chars}}][^\\s${{special_chars}}]*".to_string());
    v.insert("var_unquoted_string".to_string(), "(?:\\$\\{[^ \\}]+\\}|{{unquoted_string}})+".to_string());
    v.insert("special_chars".to_string(), "[;{}\\[\\]\"\\\\]".to_string());
    v.insert("inline_end_chars".to_string(), "[;\\s\\}\\]\\\\]".to_string());
    v.insert("most_likely_code".to_string(), "while|for|catch|return|break|continue|switch|exit|foreach|if|after|append|array|auto_execok|auto_import|auto_load|auto_mkindex|auto_mkindex_old|auto_qualify|auto_reset|bgerror|binary|cd|clock|close|concat|dde|encoding|eof|error|eval|exec|expr|fblocked|fconfigure|fcopy|file|fileevent|filename|flush|format|gets|glob|global|history|http|incr|info|interp|join|lappend|library|lindex|linsert|list|llength|load|lrange|lreplace|lsearch|lset|lsort|memory|msgcat|namespace|open|package|parray|pid|pkg::create|pkg_mkIndex|proc|puts|pwd|re_syntax|read|registry|rename|resource|scan|seek|set|socket|SafeBase|source|split|string|subst|Tcl|tcl_endOfWord|tcl_findLibrary|tcl_startOfNextWord|tcl_startOfPreviousWord|tcl_wordBreakAfter|tcl_wordBreakBefore|tcltest|tclvars|tell|time|trace|unknown|unset|update|uplevel|upvar|variable|vwait".to_string());
    v.insert("end_chars".to_string(), "[;\\n\\}\\]]".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_expr-body_0".to_string(), ContextId { index: 5719 });
    v.insert("#anon_command-name_8".to_string(), ContextId { index: 5714 });
    v.insert("proc-parameters".to_string(), ContextId { index: 5754 });
    v.insert("strings".to_string(), ContextId { index: 5757 });
    v.insert("continued-line".to_string(), ContextId { index: 5742 });
    v.insert("proc-parameter".to_string(), ContextId { index: 5753 });
    v.insert("#anon_braces_0".to_string(), ContextId { index: 5703 });
    v.insert("#anon_proc-parameters_2".to_string(), ContextId { index: 5727 });
    v.insert("commands".to_string(), ContextId { index: 5737 });
    v.insert("non-command-braces".to_string(), ContextId { index: 5748 });
    v.insert("#anon_command-name_9".to_string(), ContextId { index: 5715 });
    v.insert("#anon_proc-parameters_1".to_string(), ContextId { index: 5726 });
    v.insert("#anon_command-name_10".to_string(), ContextId { index: 5706 });
    v.insert("__start".to_string(), ContextId { index: 5733 });
    v.insert("escape".to_string(), ContextId { index: 5743 });
    v.insert("comments".to_string(), ContextId { index: 5738 });
    v.insert("#anon_command-name_1".to_string(), ContextId { index: 5705 });
    v.insert("__main".to_string(), ContextId { index: 5732 });
    v.insert("conditional-bare-expr".to_string(), ContextId { index: 5740 });
    v.insert("#anon_command-name_2".to_string(), ContextId { index: 5708 });
    v.insert("#anon_expressions_0".to_string(), ContextId { index: 5720 });
    v.insert("conditional-expr".to_string(), ContextId { index: 5741 });
    v.insert("operators".to_string(), ContextId { index: 5750 });
    v.insert("variable".to_string(), ContextId { index: 5760 });
    v.insert("#anon_command-name_3".to_string(), ContextId { index: 5709 });
    v.insert("expr-body".to_string(), ContextId { index: 5745 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 5716 });
    v.insert("#anon_proc-parameter_2".to_string(), ContextId { index: 5723 });
    v.insert("#anon_continued-line_0".to_string(), ContextId { index: 5717 });
    v.insert("main".to_string(), ContextId { index: 5747 });
    v.insert("#anon_proc-parameter_3".to_string(), ContextId { index: 5724 });
    v.insert("#anon_command-name_6".to_string(), ContextId { index: 5712 });
    v.insert("#anon_command-name_5".to_string(), ContextId { index: 5711 });
    v.insert("string-braces".to_string(), ContextId { index: 5756 });
    v.insert("#anon_continued-line_1".to_string(), ContextId { index: 5718 });
    v.insert("#anon_regexp-braces_0".to_string(), ContextId { index: 5728 });
    v.insert("#anon_command-name_0".to_string(), ContextId { index: 5704 });
    v.insert("substitution".to_string(), ContextId { index: 5758 });
    v.insert("conditional".to_string(), ContextId { index: 5739 });
    v.insert("braces".to_string(), ContextId { index: 5734 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 5730 });
    v.insert("command-braces".to_string(), ContextId { index: 5735 });
    v.insert("expr".to_string(), ContextId { index: 5744 });
    v.insert("substitution-body".to_string(), ContextId { index: 5759 });
    v.insert("numbers".to_string(), ContextId { index: 5749 });
    v.insert("command-name".to_string(), ContextId { index: 5736 });
    v.insert("#anon_proc-parameter_1".to_string(), ContextId { index: 5722 });
    v.insert("#anon_proc-parameters_0".to_string(), ContextId { index: 5725 });
    v.insert("#anon_strings_2".to_string(), ContextId { index: 5731 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 5729 });
    v.insert("expressions".to_string(), ContextId { index: 5746 });
    v.insert("proc".to_string(), ContextId { index: 5751 });
    v.insert("proc-body".to_string(), ContextId { index: 5752 });
    v.insert("regexp-braces".to_string(), ContextId { index: 5755 });
    v.insert("#anon_command-name_11".to_string(), ContextId { index: 5707 });
    v.insert("#anon_command-name_7".to_string(), ContextId { index: 5713 });
    v.insert("#anon_command-name_4".to_string(), ContextId { index: 5710 });
    v.insert("#anon_proc-parameter_0".to_string(), ContextId { index: 5721 });
    v
  }
} }