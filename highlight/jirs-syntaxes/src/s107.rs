
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Regular Expressions (Elixir)".to_string(),
  file_extensions: vec!["ex.re".to_string()],
  scope: Scope { a: 844613915181056, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("invalid_capture_name".to_string(), "[^\\[\\\\(){}|^$.?*+\\n]+".to_string());
    v.insert("ranged_quantifier".to_string(), "{\\d+,?\\d*?}".to_string());
    v.insert("capture_name".to_string(), "[a-zA-Z_][a-zA-Z_\\d]{,31}".to_string());
    v.insert("character_quantifier".to_string(), "[?*+]".to_string());
    v.insert("lazy_or_possessive".to_string(), "[?+]".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("class_set_escape_sequence".to_string(), ContextId { index: 6910 });
    v.insert("back_reference".to_string(), ContextId { index: 6904 });
    v.insert("#anon_quoted_sequence_0".to_string(), ContextId { index: 6900 });
    v.insert("#anon_group_0".to_string(), ContextId { index: 6896 });
    v.insert("__main".to_string(), ContextId { index: 6901 });
    v.insert("#anon_conditional_subpattern_2".to_string(), ContextId { index: 6894 });
    v.insert("dot_meta_char".to_string(), ContextId { index: 6916 });
    v.insert("#anon_backtracking_verb_1".to_string(), ContextId { index: 6883 });
    v.insert("#anon_posix_character_class_0".to_string(), ContextId { index: 6899 });
    v.insert("operator".to_string(), ContextId { index: 6925 });
    v.insert("conditional_subpattern_end".to_string(), ContextId { index: 6914 });
    v.insert("group_body".to_string(), ContextId { index: 6921 });
    v.insert("#anon_inline_option_0".to_string(), ContextId { index: 6898 });
    v.insert("character_property".to_string(), ContextId { index: 6907 });
    v.insert("escape_sequence".to_string(), ContextId { index: 6917 });
    v.insert("main".to_string(), ContextId { index: 6924 });
    v.insert("quantifier".to_string(), ContextId { index: 6927 });
    v.insert("unexpected_quantifier".to_string(), ContextId { index: 6930 });
    v.insert("unexpected_quantifier_pop".to_string(), ContextId { index: 6931 });
    v.insert("#anon_character_range_0".to_string(), ContextId { index: 6886 });
    v.insert("conditional_subpattern".to_string(), ContextId { index: 6913 });
    v.insert("#anon_backtracking_verb_0".to_string(), ContextId { index: 6882 });
    v.insert("#anon_character_range_1".to_string(), ContextId { index: 6887 });
    v.insert("#anon_group_1".to_string(), ContextId { index: 6897 });
    v.insert("conditional_subpattern_pop".to_string(), ContextId { index: 6915 });
    v.insert("group".to_string(), ContextId { index: 6920 });
    v.insert("backtracking_verb".to_string(), ContextId { index: 6905 });
    v.insert("common_escape_sequence".to_string(), ContextId { index: 6912 });
    v.insert("#anon_class_set_0".to_string(), ContextId { index: 6889 });
    v.insert("#anon_escaped_char_0".to_string(), ContextId { index: 6895 });
    v.insert("literal".to_string(), ContextId { index: 6923 });
    v.insert("inline_option".to_string(), ContextId { index: 6922 });
    v.insert("quoted_sequence".to_string(), ContextId { index: 6928 });
    v.insert("#anon_comment_1".to_string(), ContextId { index: 6891 });
    v.insert("class_set".to_string(), ContextId { index: 6909 });
    v.insert("__start".to_string(), ContextId { index: 6902 });
    v.insert("comment".to_string(), ContextId { index: 6911 });
    v.insert("backtracking_verb_end".to_string(), ContextId { index: 6906 });
    v.insert("expression".to_string(), ContextId { index: 6919 });
    v.insert("posix_character_class".to_string(), ContextId { index: 6926 });
    v.insert("subroutine_call".to_string(), ContextId { index: 6929 });
    v.insert("#anon_comment_0".to_string(), ContextId { index: 6890 });
    v.insert("escaped_char".to_string(), ContextId { index: 6918 });
    v.insert("#anon_conditional_subpattern_0".to_string(), ContextId { index: 6892 });
    v.insert("character_range".to_string(), ContextId { index: 6908 });
    v.insert("assertion".to_string(), ContextId { index: 6903 });
    v.insert("#anon_backtracking_verb_2".to_string(), ContextId { index: 6884 });
    v.insert("#anon_backtracking_verb_end_0".to_string(), ContextId { index: 6885 });
    v.insert("#anon_conditional_subpattern_1".to_string(), ContextId { index: 6893 });
    v.insert("#anon_character_range_2".to_string(), ContextId { index: 6888 });
    v
  }
} }