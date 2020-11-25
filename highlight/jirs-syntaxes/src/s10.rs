
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "CSS".to_string(),
  file_extensions: vec!["css".to_string(),"css.erb".to_string(),"css.liquid".to_string()],
  scope: Scope { a: 844485059674112, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("duration_units".to_string(), "(?i:s|ms)".to_string());
    v.insert("combinators".to_string(), "(?:>{1,3}|[~+])".to_string());
    v.insert("font_relative_lengths".to_string(), "(?i:em|ex|ch|rem)".to_string());
    v.insert("absolute_lengths".to_string(), "(?i:cm|mm|q|in|pt|pc|px|fr)".to_string());
    v.insert("frequency_units".to_string(), "(?i:Hz|kHz)".to_string());
    v.insert("nmchar".to_string(), "(?:[[-\\w]{{nonascii}}]|{{escape}})".to_string());
    v.insert("resolution_units".to_string(), "(?i:dpi|dpcm|dppx)".to_string());
    v.insert("unicode".to_string(), "\\\\\\h{1,6}[ \\t\\n\\f]?".to_string());
    v.insert("counter_styles".to_string(), "(?xi:\n    arabic-indic | armenian | bengali | cambodian | circle\n  | cjk-decimal | cjk-earthly-branch | cjk-heavenly-stem | decimal-leading-zero\n  | decimal | devanagari | disclosure-closed | disclosure-open | disc\n  | ethiopic-numeric | georgian | gujarati | gurmukhi | hebrew\n  | hiragana-iroha | hiragana | japanese-formal | japanese-informal\n  | kannada | katakana-iroha | katakana | khmer\n  | korean-hangul-formal | korean-hanja-formal | korean-hanja-informal | lao\n  | lower-alpha | lower-armenian | lower-greek | lower-latin | lower-roman\n  | malayalam | mongolian | myanmar | oriya | persian\n  | simp-chinese-formal | simp-chinese-informal\n  | square | tamil | telugu | thai | tibetan\n  | trad-chinese-formal | trad-chinese-informal\n  | upper-alpha | upper-armenian | upper-latin | upper-roman\n)".to_string());
    v.insert("nmstart".to_string(), "(?:[[_a-zA-Z]{{nonascii}}]|{{escape}})".to_string());
    v.insert("viewport_percentage_lengths".to_string(), "(?i:vw|vh|vmin|vmax)".to_string());
    v.insert("ident".to_string(), "(?:--{{nmchar}}+|-?{{nmstart}}{{nmchar}}*)".to_string());
    v.insert("integer".to_string(), "(?:[-+]?\\d+)".to_string());
    v.insert("angle_units".to_string(), "(?i:deg|grad|rad|turn)".to_string());
    v.insert("float".to_string(), "(?x:\n  [-+]? \\d* (\\.) \\d+ {{exponent}}?\n| [-+]? \\d+          {{exponent}}\n)".to_string());
    v.insert("exponent".to_string(), "(?:[eE]{{integer}})".to_string());
    v.insert("escape".to_string(), "(?:{{unicode}}|\\\\[^\\n\\f\\h])".to_string());
    v.insert("nonascii".to_string(), "[\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]".to_string());
    v.insert("custom_element_chars".to_string(), "(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_builtin-functions_20".to_string(), ContextId { index: 864 });
    v.insert("#anon_builtin-functions_32".to_string(), ContextId { index: 877 });
    v.insert("#anon_builtin-functions_5".to_string(), ContextId { index: 892 });
    v.insert("media-query".to_string(), ContextId { index: 1018 });
    v.insert("#anon_at-document_1".to_string(), ContextId { index: 833 });
    v.insert("#anon_literal-string_1".to_string(), ContextId { index: 942 });
    v.insert("#anon_color-functions_2".to_string(), ContextId { index: 910 });
    v.insert("pseudo-classes".to_string(), ContextId { index: 1027 });
    v.insert("#anon_at-custom-media_0".to_string(), ContextId { index: 830 });
    v.insert("#anon_image-function_1".to_string(), ContextId { index: 936 });
    v.insert("qualified-name".to_string(), ContextId { index: 1029 });
    v.insert("#anon_builtin-functions_15".to_string(), ContextId { index: 858 });
    v.insert("main".to_string(), ContextId { index: 1017 });
    v.insert("#anon_builtin-functions_6".to_string(), ContextId { index: 893 });
    v.insert("regexp-function".to_string(), ContextId { index: 1030 });
    v.insert("#anon_var-function_0".to_string(), ContextId { index: 970 });
    v.insert("line-names".to_string(), ContextId { index: 1015 });
    v.insert("resolution-type".to_string(), ContextId { index: 1031 });
    v.insert("#anon_builtin-functions_24".to_string(), ContextId { index: 868 });
    v.insert("at-page".to_string(), ContextId { index: 984 });
    v.insert("#anon_minmax-function_0".to_string(), ContextId { index: 945 });
    v.insert("frequency-type".to_string(), ContextId { index: 1005 });
    v.insert("rule-list".to_string(), ContextId { index: 1032 });
    v.insert("rule-list-body".to_string(), ContextId { index: 1033 });
    v.insert("gradient-functions".to_string(), ContextId { index: 1007 });
    v.insert("#anon_at-supports_0".to_string(), ContextId { index: 846 });
    v.insert("#anon_builtin-functions_23".to_string(), ContextId { index: 867 });
    v.insert("#anon_media-query-list_0".to_string(), ContextId { index: 943 });
    v.insert("#anon_pseudo-classes_3".to_string(), ContextId { index: 951 });
    v.insert("#anon_calc-function_0".to_string(), ContextId { index: 897 });
    v.insert("__main".to_string(), ContextId { index: 972 });
    v.insert("#anon_pseudo-classes_6".to_string(), ContextId { index: 954 });
    v.insert("#anon_builtin-functions_16".to_string(), ContextId { index: 859 });
    v.insert("#anon_gradient-functions_1".to_string(), ContextId { index: 932 });
    v.insert("#anon_selector_2".to_string(), ContextId { index: 965 });
    v.insert("#anon_var-function_1".to_string(), ContextId { index: 971 });
    v.insert("#anon_builtin-functions_36".to_string(), ContextId { index: 881 });
    v.insert("#anon_color-adjuster-functions_4".to_string(), ContextId { index: 902 });
    v.insert("#anon_regexp-function_0".to_string(), ContextId { index: 956 });
    v.insert("#anon_domain-function_0".to_string(), ContextId { index: 921 });
    v.insert("#anon_at-supports-parens_0".to_string(), ContextId { index: 845 });
    v.insert("#anon_builtin-functions_10".to_string(), ContextId { index: 853 });
    v.insert("#anon_builtin-functions_38".to_string(), ContextId { index: 883 });
    v.insert("#anon_color-functions_3".to_string(), ContextId { index: 911 });
    v.insert("#anon_color-functions_9".to_string(), ContextId { index: 917 });
    v.insert("#anon_regexp-function_1".to_string(), ContextId { index: 957 });
    v.insert("__start".to_string(), ContextId { index: 973 });
    v.insert("#anon_color-adjuster-functions_3".to_string(), ContextId { index: 901 });
    v.insert("#anon_cross-fade-function_0".to_string(), ContextId { index: 919 });
    v.insert("#anon_url-function_1".to_string(), ContextId { index: 967 });
    v.insert("#anon_builtin-functions_40".to_string(), ContextId { index: 886 });
    v.insert("at-import".to_string(), ContextId { index: 980 });
    v.insert("at-namespace".to_string(), ContextId { index: 983 });
    v.insert("#anon_at-keyframes_1".to_string(), ContextId { index: 837 });
    v.insert("at-supports".to_string(), ContextId { index: 987 });
    v.insert("#anon_builtin-functions_33".to_string(), ContextId { index: 878 });
    v.insert("#anon_builtin-functions_34".to_string(), ContextId { index: 879 });
    v.insert("#anon_builtin-functions_12".to_string(), ContextId { index: 855 });
    v.insert("#anon_filter-functions_1".to_string(), ContextId { index: 924 });
    v.insert("#anon_builtin-functions_35".to_string(), ContextId { index: 880 });
    v.insert("#anon_builtin-functions_45".to_string(), ContextId { index: 891 });
    v.insert("color-adjuster-functions".to_string(), ContextId { index: 993 });
    v.insert("#anon_image-set-function_1".to_string(), ContextId { index: 938 });
    v.insert("#anon_builtin-functions_3".to_string(), ContextId { index: 874 });
    v.insert("#anon_at-charset_0".to_string(), ContextId { index: 828 });
    v.insert("at-rules".to_string(), ContextId { index: 986 });
    v.insert("#anon_gradient-functions_2".to_string(), ContextId { index: 933 });
    v.insert("#anon_filter-functions_5".to_string(), ContextId { index: 928 });
    v.insert("custom-property-name".to_string(), ContextId { index: 1000 });
    v.insert("float-type".to_string(), ContextId { index: 1004 });
    v.insert("at-media".to_string(), ContextId { index: 982 });
    v.insert("length-type".to_string(), ContextId { index: 1014 });
    v.insert("#anon_builtin-functions_2".to_string(), ContextId { index: 863 });
    v.insert("#anon_builtin-functions_44".to_string(), ContextId { index: 890 });
    v.insert("#anon_color-adjuster-functions_9".to_string(), ContextId { index: 907 });
    v.insert("numeric-values".to_string(), ContextId { index: 1022 });
    v.insert("percentage-type".to_string(), ContextId { index: 1023 });
    v.insert("property-list".to_string(), ContextId { index: 1024 });
    v.insert("#anon_filter-functions_2".to_string(), ContextId { index: 925 });
    v.insert("#anon_gradient-functions_3".to_string(), ContextId { index: 934 });
    v.insert("time-type".to_string(), ContextId { index: 1037 });
    v.insert("#anon_rule-list-body_2".to_string(), ContextId { index: 960 });
    v.insert("integer-type".to_string(), ContextId { index: 1012 });
    v.insert("#anon_comment-block_0".to_string(), ContextId { index: 918 });
    v.insert("dimensions".to_string(), ContextId { index: 1001 });
    v.insert("#anon_filter-functions_0".to_string(), ContextId { index: 923 });
    v.insert("#anon_builtin-functions_0".to_string(), ContextId { index: 851 });
    v.insert("#anon_url-prefix-function_1".to_string(), ContextId { index: 969 });
    v.insert("property-value-constants".to_string(), ContextId { index: 1025 });
    v.insert("#anon_property-list_0".to_string(), ContextId { index: 947 });
    v.insert("#anon_builtin-functions_11".to_string(), ContextId { index: 854 });
    v.insert("#anon_rule-list-body_0".to_string(), ContextId { index: 958 });
    v.insert("#anon_url-prefix-function_0".to_string(), ContextId { index: 968 });
    v.insert("#anon_gradient-functions_0".to_string(), ContextId { index: 931 });
    v.insert("builtin-functions".to_string(), ContextId { index: 991 });
    v.insert("#anon_color-adjuster-functions_7".to_string(), ContextId { index: 905 });
    v.insert("#anon_at-counter-style_0".to_string(), ContextId { index: 829 });
    v.insert("#anon_color-functions_8".to_string(), ContextId { index: 916 });
    v.insert("#anon_media-query_0".to_string(), ContextId { index: 944 });
    v.insert("rule-list-terminator".to_string(), ContextId { index: 1034 });
    v.insert("#anon_rule-list-body_1".to_string(), ContextId { index: 959 });
    v.insert("#anon_builtin-functions_7".to_string(), ContextId { index: 894 });
    v.insert("filter-functions".to_string(), ContextId { index: 1003 });
    v.insert("#anon_filter-functions_7".to_string(), ContextId { index: 930 });
    v.insert("string-content".to_string(), ContextId { index: 1036 });
    v.insert("#anon_selector_1".to_string(), ContextId { index: 964 });
    v.insert("#anon_at-keyframes_2".to_string(), ContextId { index: 838 });
    v.insert("#anon_attr-function_1".to_string(), ContextId { index: 849 });
    v.insert("#anon_color-adjuster-functions_1".to_string(), ContextId { index: 899 });
    v.insert("url-prefix-function".to_string(), ContextId { index: 1041 });
    v.insert("#anon_builtin-functions_31".to_string(), ContextId { index: 876 });
    v.insert("#anon_at-supports_1".to_string(), ContextId { index: 847 });
    v.insert("at-counter-style".to_string(), ContextId { index: 976 });
    v.insert("#anon_filter-functions_4".to_string(), ContextId { index: 927 });
    v.insert("#anon_attr-function_2".to_string(), ContextId { index: 850 });
    v.insert("#anon_builtin-functions_18".to_string(), ContextId { index: 861 });
    v.insert("#anon_filter-functions_6".to_string(), ContextId { index: 929 });
    v.insert("#anon_pseudo-classes_0".to_string(), ContextId { index: 948 });
    v.insert("#anon_at-page_0".to_string(), ContextId { index: 844 });
    v.insert("#anon_pseudo-classes_2".to_string(), ContextId { index: 950 });
    v.insert("#anon_selector_0".to_string(), ContextId { index: 963 });
    v.insert("#anon_pseudo-classes_5".to_string(), ContextId { index: 953 });
    v.insert("#anon_builtin-functions_42".to_string(), ContextId { index: 888 });
    v.insert("at-supports-operators".to_string(), ContextId { index: 988 });
    v.insert("keyframe-name".to_string(), ContextId { index: 1013 });
    v.insert("#anon_builtin-functions_26".to_string(), ContextId { index: 870 });
    v.insert("image-type".to_string(), ContextId { index: 1010 });
    v.insert("inside-calc-parens".to_string(), ContextId { index: 1011 });
    v.insert("angle-type".to_string(), ContextId { index: 974 });
    v.insert("at-rule-punctuation".to_string(), ContextId { index: 985 });
    v.insert("at-document".to_string(), ContextId { index: 978 });
    v.insert("#anon_builtin-functions_25".to_string(), ContextId { index: 869 });
    v.insert("#anon_builtin-functions_41".to_string(), ContextId { index: 887 });
    v.insert("number-type".to_string(), ContextId { index: 1021 });
    v.insert("#anon_builtin-functions_39".to_string(), ContextId { index: 884 });
    v.insert("calc-function".to_string(), ContextId { index: 992 });
    v.insert("image-set-function".to_string(), ContextId { index: 1009 });
    v.insert("pseudo-elements".to_string(), ContextId { index: 1028 });
    v.insert("#anon_image-function_0".to_string(), ContextId { index: 935 });
    v.insert("unicode-range".to_string(), ContextId { index: 1038 });
    v.insert("minmax-function".to_string(), ContextId { index: 1020 });
    v.insert("comma-delimiter".to_string(), ContextId { index: 997 });
    v.insert("#anon_builtin-functions_9".to_string(), ContextId { index: 896 });
    v.insert("#anon_attr-function_0".to_string(), ContextId { index: 848 });
    v.insert("#anon_color-adjuster-functions_2".to_string(), ContextId { index: 900 });
    v.insert("#anon_builtin-functions_30".to_string(), ContextId { index: 875 });
    v.insert("#anon_color-adjuster-functions_5".to_string(), ContextId { index: 903 });
    v.insert("#anon_cross-fade-function_1".to_string(), ContextId { index: 920 });
    v.insert("#anon_at-keyframes_3".to_string(), ContextId { index: 839 });
    v.insert("#anon_at-media_1".to_string(), ContextId { index: 841 });
    v.insert("#anon_pseudo-classes_4".to_string(), ContextId { index: 952 });
    v.insert("#anon_image-set-function_0".to_string(), ContextId { index: 937 });
    v.insert("at-font-face".to_string(), ContextId { index: 979 });
    v.insert("#anon_builtin-functions_29".to_string(), ContextId { index: 873 });
    v.insert("#anon_at-keyframes_0".to_string(), ContextId { index: 836 });
    v.insert("domain-function".to_string(), ContextId { index: 1002 });
    v.insert("#anon_builtin-functions_37".to_string(), ContextId { index: 882 });
    v.insert("#anon_line-names_0".to_string(), ContextId { index: 940 });
    v.insert("#anon_at-document_0".to_string(), ContextId { index: 832 });
    v.insert("#anon_color-functions_7".to_string(), ContextId { index: 915 });
    v.insert("#anon_rule-list-body_3".to_string(), ContextId { index: 961 });
    v.insert("#anon_color-functions_4".to_string(), ContextId { index: 912 });
    v.insert("#anon_builtin-functions_13".to_string(), ContextId { index: 856 });
    v.insert("#anon_color-functions_0".to_string(), ContextId { index: 908 });
    v.insert("at-keyframes".to_string(), ContextId { index: 981 });
    v.insert("#anon_at-custom-media_1".to_string(), ContextId { index: 831 });
    v.insert("color-functions".to_string(), ContextId { index: 995 });
    v.insert("cross-fade-function".to_string(), ContextId { index: 999 });
    v.insert("property-values".to_string(), ContextId { index: 1026 });
    v.insert("#anon_color-adjuster-functions_0".to_string(), ContextId { index: 898 });
    v.insert("#anon_builtin-functions_22".to_string(), ContextId { index: 866 });
    v.insert("#anon_color-adjuster-functions_8".to_string(), ContextId { index: 906 });
    v.insert("#anon_builtin-functions_27".to_string(), ContextId { index: 871 });
    v.insert("#anon_at-import_0".to_string(), ContextId { index: 835 });
    v.insert("#anon_builtin-functions_4".to_string(), ContextId { index: 885 });
    v.insert("#anon_pseudo-classes_1".to_string(), ContextId { index: 949 });
    v.insert("color-values".to_string(), ContextId { index: 996 });
    v.insert("#anon_color-adjuster-functions_6".to_string(), ContextId { index: 904 });
    v.insert("comment-block".to_string(), ContextId { index: 998 });
    v.insert("function-notation-terminator".to_string(), ContextId { index: 1006 });
    v.insert("#anon_builtin-functions_19".to_string(), ContextId { index: 862 });
    v.insert("#anon_minmax-function_1".to_string(), ContextId { index: 946 });
    v.insert("media-query-list".to_string(), ContextId { index: 1019 });
    v.insert("#anon_builtin-functions_17".to_string(), ContextId { index: 860 });
    v.insert("unquoted-string".to_string(), ContextId { index: 1039 });
    v.insert("color-adjuster-operators".to_string(), ContextId { index: 994 });
    v.insert("#anon_at-media_2".to_string(), ContextId { index: 842 });
    v.insert("#anon_url-function_0".to_string(), ContextId { index: 966 });
    v.insert("selector".to_string(), ContextId { index: 1035 });
    v.insert("#anon_builtin-functions_21".to_string(), ContextId { index: 865 });
    v.insert("image-function".to_string(), ContextId { index: 1008 });
    v.insert("url-function".to_string(), ContextId { index: 1040 });
    v.insert("#anon_at-namespace_0".to_string(), ContextId { index: 843 });
    v.insert("#anon_builtin-functions_28".to_string(), ContextId { index: 872 });
    v.insert("#anon_domain-function_1".to_string(), ContextId { index: 922 });
    v.insert("#anon_color-functions_1".to_string(), ContextId { index: 909 });
    v.insert("attr-function".to_string(), ContextId { index: 990 });
    v.insert("var-function".to_string(), ContextId { index: 1042 });
    v.insert("vendor-prefix".to_string(), ContextId { index: 1043 });
    v.insert("literal-string".to_string(), ContextId { index: 1016 });
    v.insert("#anon_at-media_0".to_string(), ContextId { index: 840 });
    v.insert("#anon_filter-functions_3".to_string(), ContextId { index: 926 });
    v.insert("#anon_keyframe-name_0".to_string(), ContextId { index: 939 });
    v.insert("#anon_color-functions_5".to_string(), ContextId { index: 913 });
    v.insert("#anon_builtin-functions_14".to_string(), ContextId { index: 857 });
    v.insert("#anon_literal-string_0".to_string(), ContextId { index: 941 });
    v.insert("#anon_rule-list_0".to_string(), ContextId { index: 962 });
    v.insert("#anon_builtin-functions_43".to_string(), ContextId { index: 889 });
    v.insert("#anon_pseudo-classes_7".to_string(), ContextId { index: 955 });
    v.insert("at-charset".to_string(), ContextId { index: 975 });
    v.insert("#anon_builtin-functions_8".to_string(), ContextId { index: 895 });
    v.insert("at-custom-media".to_string(), ContextId { index: 977 });
    v.insert("#anon_at-font-face_0".to_string(), ContextId { index: 834 });
    v.insert("#anon_builtin-functions_1".to_string(), ContextId { index: 852 });
    v.insert("#anon_color-functions_6".to_string(), ContextId { index: 914 });
    v.insert("at-supports-parens".to_string(), ContextId { index: 989 });
    v
  }
} }