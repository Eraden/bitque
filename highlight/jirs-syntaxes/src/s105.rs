
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Elixir".to_string(),
  file_extensions: vec!["ex".to_string(),"exs".to_string()],
  scope: Scope { a: 844850131894272, b: 0 },
  first_line_match: Some("^#!/.*\\b(?:elixirc?|iex)".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("id_begin".to_string(), "[[:alpha:]_]".to_string());
    v.insert("module_name".to_string(), "\\b[A-Z][a-zA-Z0-9_]*\\b".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_string_5".to_string(), ContextId { index: 6829 });
    v.insert("#anon_string_26".to_string(), ContextId { index: 6803 });
    v.insert("#anon_heredoc_string_raw_1".to_string(), ContextId { index: 6776 });
    v.insert("regex_elixir".to_string(), ContextId { index: 6861 });
    v.insert("#anon_string_18".to_string(), ContextId { index: 6794 });
    v.insert("#anon_heredoc_string_raw_0".to_string(), ContextId { index: 6775 });
    v.insert("#anon_string_8".to_string(), ContextId { index: 6839 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 6781 });
    v.insert("#anon_heredoc_regex_interpolated_2".to_string(), ContextId { index: 6767 });
    v.insert("#anon_string_11".to_string(), ContextId { index: 6787 });
    v.insert("#anon_string_54".to_string(), ContextId { index: 6834 });
    v.insert("function_body".to_string(), ContextId { index: 6850 });
    v.insert("#anon_string_12".to_string(), ContextId { index: 6788 });
    v.insert("#anon_string_7".to_string(), ContextId { index: 6838 });
    v.insert("heredoc_regex_interpolated".to_string(), ContextId { index: 6851 });
    v.insert("#anon_string_16".to_string(), ContextId { index: 6792 });
    v.insert("#anon_string_25".to_string(), ContextId { index: 6802 });
    v.insert("#anon_string_41".to_string(), ContextId { index: 6820 });
    v.insert("#anon_string_42".to_string(), ContextId { index: 6821 });
    v.insert("#anon_heredoc_regex_raw_1".to_string(), ContextId { index: 6770 });
    v.insert("#anon_core_syntax_1".to_string(), ContextId { index: 6755 });
    v.insert("string_closing_angle".to_string(), ContextId { index: 6865 });
    v.insert("#anon_string_37".to_string(), ContextId { index: 6815 });
    v.insert("core_syntax".to_string(), ContextId { index: 6845 });
    v.insert("#anon_string_46".to_string(), ContextId { index: 6825 });
    v.insert("string_closing_curly".to_string(), ContextId { index: 6866 });
    v.insert("#anon_heredoc_regex_interpolated_0".to_string(), ContextId { index: 6765 });
    v.insert("#anon_string_17".to_string(), ContextId { index: 6793 });
    v.insert("#anon_string_1".to_string(), ContextId { index: 6785 });
    v.insert("#anon_interpolated_elixir_1".to_string(), ContextId { index: 6778 });
    v.insert("#anon_string_9".to_string(), ContextId { index: 6840 });
    v.insert("#anon_heredoc_regex_interpolated_1".to_string(), ContextId { index: 6766 });
    v.insert("escaped_or_interpolated".to_string(), ContextId { index: 6849 });
    v.insert("#anon_string_56".to_string(), ContextId { index: 6836 });
    v.insert("heredoc_string_closing_single".to_string(), ContextId { index: 6854 });
    v.insert("#anon_string_34".to_string(), ContextId { index: 6812 });
    v.insert("#anon_heredoc_regex_raw_0".to_string(), ContextId { index: 6769 });
    v.insert("heredoc_string_closing_double".to_string(), ContextId { index: 6853 });
    v.insert("#anon_string_13".to_string(), ContextId { index: 6789 });
    v.insert("#anon_string_49".to_string(), ContextId { index: 6828 });
    v.insert("heredoc_string_interpolated".to_string(), ContextId { index: 6855 });
    v.insert("tuple".to_string(), ContextId { index: 6871 });
    v.insert("#anon_string_47".to_string(), ContextId { index: 6826 });
    v.insert("#anon_string_20".to_string(), ContextId { index: 6797 });
    v.insert("punctuations".to_string(), ContextId { index: 6860 });
    v.insert("#anon_string_50".to_string(), ContextId { index: 6830 });
    v.insert("#anon_heredoc_string_interpolated_1".to_string(), ContextId { index: 6774 });
    v.insert("docs".to_string(), ContextId { index: 6847 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6780 });
    v.insert("#anon_string_21".to_string(), ContextId { index: 6798 });
    v.insert("#anon_interpolated_elixir_0".to_string(), ContextId { index: 6777 });
    v.insert("#anon_string_45".to_string(), ContextId { index: 6824 });
    v.insert("heredoc_string_raw".to_string(), ContextId { index: 6856 });
    v.insert("#anon_string_35".to_string(), ContextId { index: 6813 });
    v.insert("#anon_def_blocks_1".to_string(), ContextId { index: 6757 });
    v.insert("#anon_string_19".to_string(), ContextId { index: 6795 });
    v.insert("#anon_docs_1".to_string(), ContextId { index: 6760 });
    v.insert("#anon_binary_1".to_string(), ContextId { index: 6753 });
    v.insert("#anon_string_30".to_string(), ContextId { index: 6808 });
    v.insert("string_modifiers".to_string(), ContextId { index: 6869 });
    v.insert("#anon_string_2".to_string(), ContextId { index: 6796 });
    v.insert("#anon_heredoc_regex_raw_3".to_string(), ContextId { index: 6772 });
    v.insert("#anon_string_36".to_string(), ContextId { index: 6814 });
    v.insert("__start".to_string(), ContextId { index: 6843 });
    v.insert("string_closing_square".to_string(), ContextId { index: 6868 });
    v.insert("simple_string".to_string(), ContextId { index: 6863 });
    v.insert("def_blocks".to_string(), ContextId { index: 6846 });
    v.insert("#anon_string_44".to_string(), ContextId { index: 6823 });
    v.insert("string".to_string(), ContextId { index: 6864 });
    v.insert("#anon_string_3".to_string(), ContextId { index: 6807 });
    v.insert("#anon_string_51".to_string(), ContextId { index: 6831 });
    v.insert("#anon_docs_0".to_string(), ContextId { index: 6759 });
    v.insert("heredoc_regex_raw".to_string(), ContextId { index: 6852 });
    v.insert("#anon_string_55".to_string(), ContextId { index: 6835 });
    v.insert("main".to_string(), ContextId { index: 6858 });
    v.insert("#anon_string_43".to_string(), ContextId { index: 6822 });
    v.insert("#anon_string_52".to_string(), ContextId { index: 6832 });
    v.insert("#anon_simple_string_0".to_string(), ContextId { index: 6782 });
    v.insert("numeric".to_string(), ContextId { index: 6859 });
    v.insert("#anon_tuple_0".to_string(), ContextId { index: 6841 });
    v.insert("string_closing_round".to_string(), ContextId { index: 6867 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 6784 });
    v.insert("regex_or_interpolated".to_string(), ContextId { index: 6862 });
    v.insert("#anon_core_syntax_0".to_string(), ContextId { index: 6754 });
    v.insert("#anon_string_48".to_string(), ContextId { index: 6827 });
    v.insert("#anon_string_10".to_string(), ContextId { index: 6786 });
    v.insert("#anon_function_body_0".to_string(), ContextId { index: 6764 });
    v.insert("#anon_string_40".to_string(), ContextId { index: 6819 });
    v.insert("#anon_string_24".to_string(), ContextId { index: 6801 });
    v.insert("#anon_docs_4".to_string(), ContextId { index: 6763 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6779 });
    v.insert("#anon_docs_2".to_string(), ContextId { index: 6761 });
    v.insert("#anon_string_32".to_string(), ContextId { index: 6810 });
    v.insert("#anon_string_39".to_string(), ContextId { index: 6817 });
    v.insert("#anon_string_15".to_string(), ContextId { index: 6791 });
    v.insert("#anon_string_29".to_string(), ContextId { index: 6806 });
    v.insert("#anon_string_53".to_string(), ContextId { index: 6833 });
    v.insert("interpolated_elixir".to_string(), ContextId { index: 6857 });
    v.insert("#anon_string_14".to_string(), ContextId { index: 6790 });
    v.insert("string_modifiers_and_pop".to_string(), ContextId { index: 6870 });
    v.insert("#anon_string_28".to_string(), ContextId { index: 6805 });
    v.insert("#anon_string_38".to_string(), ContextId { index: 6816 });
    v.insert("#anon_binary_0".to_string(), ContextId { index: 6752 });
    v.insert("binary".to_string(), ContextId { index: 6844 });
    v.insert("escaped_char".to_string(), ContextId { index: 6848 });
    v.insert("#anon_string_22".to_string(), ContextId { index: 6799 });
    v.insert("__main".to_string(), ContextId { index: 6842 });
    v.insert("#anon_string_33".to_string(), ContextId { index: 6811 });
    v.insert("#anon_def_blocks_2".to_string(), ContextId { index: 6758 });
    v.insert("#anon_heredoc_regex_raw_2".to_string(), ContextId { index: 6771 });
    v.insert("#anon_docs_3".to_string(), ContextId { index: 6762 });
    v.insert("#anon_simple_string_1".to_string(), ContextId { index: 6783 });
    v.insert("#anon_def_blocks_0".to_string(), ContextId { index: 6756 });
    v.insert("#anon_string_23".to_string(), ContextId { index: 6800 });
    v.insert("#anon_heredoc_regex_interpolated_3".to_string(), ContextId { index: 6768 });
    v.insert("#anon_heredoc_string_interpolated_0".to_string(), ContextId { index: 6773 });
    v.insert("#anon_string_27".to_string(), ContextId { index: 6804 });
    v.insert("#anon_string_4".to_string(), ContextId { index: 6818 });
    v.insert("#anon_string_6".to_string(), ContextId { index: 6837 });
    v.insert("#anon_string_31".to_string(), ContextId { index: 6809 });
    v
  }
} }