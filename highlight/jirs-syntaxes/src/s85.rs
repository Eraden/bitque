
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "YAML".to_string(),
  file_extensions: vec!["yaml".to_string(),"yml".to_string(),"sublime-syntax".to_string()],
  scope: Scope { a: 844772822482944, b: 0 },
  first_line_match: Some("^%YAML( ?1.\\d+)?".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("ns_anchor_char".to_string(), "[^\\s\\[\\]/{/},]".to_string());
    v.insert("_type_float".to_string(), "(?x:\n    ([-+]? (?: [0-9] [0-9_]*)? (\\.) [0-9.]* (?: [eE] [-+] [0-9]+)?) # (base 10)\n  | ([-+]? [0-9] [0-9_]* (?: :[0-5]?[0-9])+ (\\.) [0-9_]*) # (base 60)\n  | ([-+]? (\\.) (?: inf|Inf|INF)) # (infinity)\n  | (      (\\.) (?: nan|NaN|NAN)) # (not a number)\n)".to_string());
    v.insert("_type_int".to_string(), "(?x:\n    ([-+]? (0b) [0-1_]+) # (base 2)\n  | ([-+]? (0)  [0-7_]+) # (base 8)\n  | ([-+]? (?: 0|[1-9][0-9_]*)) # (base 10)\n  | ([-+]? (0x) [0-9a-fA-F_]+) # (base 16)\n  | ([-+]? [1-9] [0-9_]* (?: :[0-5]?[0-9])+) # (base 60)\n)".to_string());
    v.insert("c_flow_indicator".to_string(), "[\\[\\]{},]".to_string());
    v.insert("_flow_scalar_end_plain_out".to_string(), "(?x:\n  (?=\n      \\s* $\n    | \\s+ \\#\n    | \\s* : (\\s|$)\n  )\n)".to_string());
    v.insert("_type_bool".to_string(), "(?x:\n   y|Y|yes|Yes|YES|n|N|no|No|NO\n  |true|True|TRUE|false|False|FALSE\n  |on|On|ON|off|Off|OFF\n)".to_string());
    v.insert("ns_plain_first_plain_out".to_string(), "(?x:\n    [^\\s{{c_indicator}}]\n  | [?:-] \\S\n)".to_string());
    v.insert("_type_value".to_string(), "=".to_string());
    v.insert("c_ns_esc_char".to_string(), "\\\\(?:[0abtnvfre \"/\\\\N_Lp]|x[\\dA-Fa-f]{2}|u[\\dA-Fa-f]{4}|U[\\dA-Fa-f]{8})".to_string());
    v.insert("ns_anchor_name".to_string(), "{{ns_anchor_char}}+".to_string());
    v.insert("ns_plain_first_plain_in".to_string(), "(?x:\n    [^\\s{{c_indicator}}]\n  | [?:-] [^\\s{{c_flow_indicator}}]\n)".to_string());
    v.insert("ns_uri_char".to_string(), "(?x: %[0-9A-Fa-f]{2} | [0-9A-Za-z\\-#;/?:@&=+$,_.!~*\'()\\[\\]] )".to_string());
    v.insert("_type_all".to_string(), "(?x:\n    ({{_type_null}})\n  | ({{_type_bool}})\n  | ({{_type_int}})\n  | ({{_type_float}})\n  | ({{_type_timestamp}})\n  | ({{_type_value}})\n  | ({{_type_merge}})\n )".to_string());
    v.insert("ns_tag_char".to_string(), "(?x: %[0-9A-Fa-f]{2} | [0-9A-Za-z\\-#;/?:@&=+$_.~*\'()] )".to_string());
    v.insert("c_ns_tag_property".to_string(), "(?x:\n    ! < {{ns_uri_char}}+ >\n  | {{c_tag_handle}} {{ns_tag_char}}+\n  | !\n)".to_string());
    v.insert("ns_word_char".to_string(), "[0-9A-Za-z\\-]".to_string());
    v.insert("_type_null".to_string(), "(?:null|Null|NULL|~)".to_string());
    v.insert("ns_tag_prefix".to_string(), "(?x:\n    !              {{ns_uri_char}}*\n  | (?![,!\\[\\]{}]) {{ns_uri_char}}+\n)".to_string());
    v.insert("_type_merge".to_string(), "<<".to_string());
    v.insert("s_sep".to_string(), "[ \\t]+".to_string());
    v.insert("_flow_scalar_end_plain_in".to_string(), "(?x:\n  (?=\n      \\s* $\n    | \\s+ \\#\n    | \\s* : (\\s|$)\n    | \\s* : {{c_flow_indicator}}\n    | \\s* {{c_flow_indicator}}\n  )\n)".to_string());
    v.insert("c_indicator".to_string(), "[-?:,\\[\\]{}#&*!|>\'\"%@`]".to_string());
    v.insert("_type_timestamp".to_string(), "(?x:\n    \\d{4} (-) \\d{2} (-) \\d{2}       # (y-m-d)\n  | \\d{4}                           # (year)\n    (-) \\d{1,2}                     # (month)\n    (-) \\d{1,2}                     # (day)\n    (?: [Tt] | [ \\t]+) \\d{1,2}      # (hour)\n    (:) \\d{2}                       # (minute)\n    (:) \\d{2}                       # (second)\n    (?: (\\.)\\d*)?                   # (fraction)\n    [ \\t]*\n    (?:\n      Z | [-+] \\d{1,2} (?: (:)\\d{1,2})?\n    )?                              # (time zone)\n)".to_string());
    v.insert("c_tag_handle".to_string(), "(?:!(?:{{ns_word_char}}*!)?)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("flow-scalar".to_string(), ContextId { index: 5873 });
    v.insert("#anon_flow-pair_1".to_string(), ContextId { index: 5848 });
    v.insert("block-scalar".to_string(), ContextId { index: 5862 });
    v.insert("flow-scalar-plain-in".to_string(), ContextId { index: 5875 });
    v.insert("flow-scalar-double-quoted".to_string(), ContextId { index: 5874 });
    v.insert("#anon_block-pair_0".to_string(), ContextId { index: 5838 });
    v.insert("flow-alias".to_string(), ContextId { index: 5867 });
    v.insert("flow-scalar-plain-out-implicit-type".to_string(), ContextId { index: 5878 });
    v.insert("property".to_string(), ContextId { index: 5883 });
    v.insert("__main".to_string(), ContextId { index: 5856 });
    v.insert("#anon_flow-scalar-double-quoted_0".to_string(), ContextId { index: 5850 });
    v.insert("flow-pair-value".to_string(), ContextId { index: 5872 });
    v.insert("#anon_block-pair_1".to_string(), ContextId { index: 5839 });
    v.insert("block-node".to_string(), ContextId { index: 5860 });
    v.insert("#anon_flow-pair_0".to_string(), ContextId { index: 5847 });
    v.insert("__start".to_string(), ContextId { index: 5857 });
    v.insert("block-collection".to_string(), ContextId { index: 5858 });
    v.insert("block-pair".to_string(), ContextId { index: 5861 });
    v.insert("directive".to_string(), ContextId { index: 5865 });
    v.insert("#anon_flow-mapping_0".to_string(), ContextId { index: 5846 });
    v.insert("#anon_flow-sequence_0".to_string(), ContextId { index: 5854 });
    v.insert("block-mapping".to_string(), ContextId { index: 5859 });
    v.insert("#anon_comment_0".to_string(), ContextId { index: 5843 });
    v.insert("flow-node".to_string(), ContextId { index: 5870 });
    v.insert("flow-pair".to_string(), ContextId { index: 5871 });
    v.insert("node".to_string(), ContextId { index: 5882 });
    v.insert("prototype".to_string(), ContextId { index: 5884 });
    v.insert("directive-finish".to_string(), ContextId { index: 5866 });
    v.insert("#anon_comment_1".to_string(), ContextId { index: 5844 });
    v.insert("#anon_flow-scalar-plain-out_0".to_string(), ContextId { index: 5852 });
    v.insert("flow-sequence".to_string(), ContextId { index: 5880 });
    v.insert("#anon_flow-scalar-single-quoted_0".to_string(), ContextId { index: 5853 });
    v.insert("#anon_block-scalar_0".to_string(), ContextId { index: 5841 });
    v.insert("#anon_directive_0".to_string(), ContextId { index: 5845 });
    v.insert("block-sequence".to_string(), ContextId { index: 5863 });
    v.insert("flow-scalar-plain-out".to_string(), ContextId { index: 5877 });
    v.insert("main".to_string(), ContextId { index: 5881 });
    v.insert("#anon_block-pair_2".to_string(), ContextId { index: 5840 });
    v.insert("#anon_block-scalar_1".to_string(), ContextId { index: 5842 });
    v.insert("#anon_flow-pair_2".to_string(), ContextId { index: 5849 });
    v.insert("flow-scalar-plain-in-implicit-type".to_string(), ContextId { index: 5876 });
    v.insert("flow-scalar-single-quoted".to_string(), ContextId { index: 5879 });
    v.insert("#anon_flow-scalar-plain-in_0".to_string(), ContextId { index: 5851 });
    v.insert("#anon_property_0".to_string(), ContextId { index: 5855 });
    v.insert("comment".to_string(), ContextId { index: 5864 });
    v.insert("flow-collection".to_string(), ContextId { index: 5868 });
    v.insert("flow-mapping".to_string(), ContextId { index: 5869 });
    v
  }
} }