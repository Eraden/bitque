
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Lua".to_string(),
  file_extensions: vec!["lua".to_string()],
  scope: Scope { a: 844626793594880, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier_break".to_string(), "(?!{{identifier_char}})".to_string());
    v.insert("reserved_word_statement".to_string(), "(?x:(?:\n  and|break|do|elseif|else|end|for|goto|if|in|\n  local|or|repeat|return|then|until|while\n){{identifier_break}})".to_string());
    v.insert("function_args_begin".to_string(), "(?:\\(|\"|\'|\\[=*\\[|\\{)".to_string());
    v.insert("dec_exponent".to_string(), "(?:[Ee][-+]?\\d*)".to_string());
    v.insert("identifier_char".to_string(), "(?:[A-Za-z0-9_])".to_string());
    v.insert("identifier_raw".to_string(), "(?:{{identifier_start}}{{identifier_char}}*)".to_string());
    v.insert("metamethod".to_string(), "(?x:__(?:\n  # special\n  index|newindex|call|tostring|len|i?pairs|gc\n  # math operators\n  |unm|add|sub|mul|i?div|mod|pow|concat\n  # bitwise operators\n  |band|bor|bxor|bnot|shl|shr\n  # comparison\n  |eq|lt|le\n){{identifier_break}})".to_string());
    v.insert("hex_exponent".to_string(), "(?:[Pp][-+]?\\d*)".to_string());
    v.insert("identifier".to_string(), "(?:(?!{{reserved_word}}){{identifier_raw}})".to_string());
    v.insert("identifier_start".to_string(), "(?:[A-Za-z_])".to_string());
    v.insert("metaproperty".to_string(), "(?:__(?:metatable|mode){{identifier_break}})".to_string());
    v.insert("function_assignment_ahead".to_string(), "(?=\\s*=\\s*function{{identifier_break}})".to_string());
    v.insert("reserved_word".to_string(), "(?x:(?:\n  and|break|do|elseif|else|end|false|for|function|goto|if|in|\n  local|nil|not|or|repeat|return|then|true|until|while\n){{identifier_break}})".to_string());
    v.insert("function_call_ahead".to_string(), "(?=\\s*{{function_args_begin}})".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_builtin-modules_14".to_string(), ContextId { index: 3044 });
    v.insert("expression-list".to_string(), ContextId { index: 3091 });
    v.insert("function-arguments".to_string(), ContextId { index: 3093 });
    v.insert("prefix-operator".to_string(), ContextId { index: 3109 });
    v.insert("#anon_control-statements_0".to_string(), ContextId { index: 3058 });
    v.insert("#anon_string-content_0".to_string(), ContextId { index: 3071 });
    v.insert("#anon_builtin-modules_11".to_string(), ContextId { index: 3041 });
    v.insert("#anon_builtin-modules_12".to_string(), ContextId { index: 3042 });
    v.insert("#anon_builtin-modules_16".to_string(), ContextId { index: 3046 });
    v.insert("#anon_control-statements_2".to_string(), ContextId { index: 3060 });
    v.insert("#anon_builtin-modules_2".to_string(), ContextId { index: 3050 });
    v.insert("#anon_table-constructor_2".to_string(), ContextId { index: 3077 });
    v.insert("builtin-modules".to_string(), ContextId { index: 3084 });
    v.insert("function-literal".to_string(), ContextId { index: 3095 });
    v.insert("prototype".to_string(), ContextId { index: 3111 });
    v.insert("builtin".to_string(), ContextId { index: 3083 });
    v.insert("function-name-property".to_string(), ContextId { index: 3100 });
    v.insert("#anon_builtin-modules_9".to_string(), ContextId { index: 3057 });
    v.insert("string-content".to_string(), ContextId { index: 3116 });
    v.insert("reserved-word-pop".to_string(), ContextId { index: 3113 });
    v.insert("infix-operator".to_string(), ContextId { index: 3105 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 3072 });
    v.insert("#anon_builtin-modules_19".to_string(), ContextId { index: 3049 });
    v.insert("control-statements".to_string(), ContextId { index: 3085 });
    v.insert("end".to_string(), ContextId { index: 3087 });
    v.insert("#anon_builtin-modules_4".to_string(), ContextId { index: 3052 });
    v.insert("function-meta".to_string(), ContextId { index: 3096 });
    v.insert("immediately-pop".to_string(), ContextId { index: 3104 });
    v.insert("#anon_builtin-modules_13".to_string(), ContextId { index: 3043 });
    v.insert("#anon_builtin-modules_17".to_string(), ContextId { index: 3047 });
    v.insert("#anon_control-statements_3".to_string(), ContextId { index: 3061 });
    v.insert("#anon_control-statements_5".to_string(), ContextId { index: 3063 });
    v.insert("#anon_control-statements_7".to_string(), ContextId { index: 3065 });
    v.insert("#anon_function-arguments_0".to_string(), ContextId { index: 3066 });
    v.insert("if-then".to_string(), ContextId { index: 3103 });
    v.insert("main".to_string(), ContextId { index: 3106 });
    v.insert("#anon_builtin-modules_15".to_string(), ContextId { index: 3045 });
    v.insert("expression-list-end".to_string(), ContextId { index: 3092 });
    v.insert("#anon_table-constructor_0".to_string(), ContextId { index: 3075 });
    v.insert("reserved-word-expression-pop".to_string(), ContextId { index: 3112 });
    v.insert("function-parameter-list".to_string(), ContextId { index: 3101 });
    v.insert("#anon_function-parameter-list_0".to_string(), ContextId { index: 3067 });
    v.insert("function-name".to_string(), ContextId { index: 3097 });
    v.insert("expression-end".to_string(), ContextId { index: 3090 });
    v.insert("if-block".to_string(), ContextId { index: 3102 });
    v.insert("#anon_builtin-modules_18".to_string(), ContextId { index: 3048 });
    v.insert("#anon_table-constructor_3".to_string(), ContextId { index: 3078 });
    v.insert("__start".to_string(), ContextId { index: 3080 });
    v.insert("statements".to_string(), ContextId { index: 3114 });
    v.insert("variable".to_string(), ContextId { index: 3119 });
    v.insert("parenthesized-expression".to_string(), ContextId { index: 3108 });
    v.insert("#anon_table-constructor_1".to_string(), ContextId { index: 3076 });
    v.insert("#anon_prototype_0".to_string(), ContextId { index: 3069 });
    v.insert("expression-begin".to_string(), ContextId { index: 3089 });
    v.insert("#anon_builtin-modules_6".to_string(), ContextId { index: 3054 });
    v.insert("__main".to_string(), ContextId { index: 3079 });
    v.insert("#anon_builtin-modules_3".to_string(), ContextId { index: 3051 });
    v.insert("#anon_control-statements_1".to_string(), ContextId { index: 3059 });
    v.insert("function-name-end".to_string(), ContextId { index: 3099 });
    v.insert("function-arguments-meta".to_string(), ContextId { index: 3094 });
    v.insert("support".to_string(), ContextId { index: 3117 });
    v.insert("function-name-begin".to_string(), ContextId { index: 3098 });
    v.insert("#anon_accessor_0".to_string(), ContextId { index: 3037 });
    v.insert("#anon_builtin-modules_0".to_string(), ContextId { index: 3038 });
    v.insert("#anon_builtin-modules_5".to_string(), ContextId { index: 3053 });
    v.insert("#anon_control-statements_4".to_string(), ContextId { index: 3062 });
    v.insert("#anon_control-statements_6".to_string(), ContextId { index: 3064 });
    v.insert("#anon_string_1".to_string(), ContextId { index: 3073 });
    v.insert("#anon_parenthesized-expression_0".to_string(), ContextId { index: 3068 });
    v.insert("#anon_string_2".to_string(), ContextId { index: 3074 });
    v.insert("block-contents".to_string(), ContextId { index: 3082 });
    v.insert("string".to_string(), ContextId { index: 3115 });
    v.insert("property".to_string(), ContextId { index: 3110 });
    v.insert("#anon_builtin-modules_1".to_string(), ContextId { index: 3039 });
    v.insert("#anon_builtin-modules_8".to_string(), ContextId { index: 3056 });
    v.insert("else-pop".to_string(), ContextId { index: 3086 });
    v.insert("expression".to_string(), ContextId { index: 3088 });
    v.insert("number".to_string(), ContextId { index: 3107 });
    v.insert("#anon_builtin-modules_7".to_string(), ContextId { index: 3055 });
    v.insert("table-constructor".to_string(), ContextId { index: 3118 });
    v.insert("#anon_builtin-modules_10".to_string(), ContextId { index: 3040 });
    v.insert("#anon_prototype_1".to_string(), ContextId { index: 3070 });
    v.insert("accessor".to_string(), ContextId { index: 3081 });
    v
  }
} }