
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Assembly (x86_64)".to_string(),
  file_extensions: vec!["yasm".to_string(),"nasm".to_string(),"asm".to_string(),"inc".to_string(),"mac".to_string()],
  scope: Scope { a: 844790008053760, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("num_hex".to_string(), "(?:[[:xdigit:]][[:xdigit:]_]*)".to_string());
    v.insert("num_bin_exp".to_string(), "(?:p[+-]?{{num_dec}})".to_string());
    v.insert("valid_identifier".to_string(), "(?:[[:alpha:]_?]{{identifier_body}})".to_string());
    v.insert("num_bin".to_string(), "(?:[01][01_]*)".to_string());
    v.insert("identifier_body".to_string(), "(?:[[:alnum:]_$#@~.?]*)".to_string());
    v.insert("num_dec".to_string(), "(?:[0-9][0-9_]*)".to_string());
    v.insert("num_oct".to_string(), "(?:[0-7][0-7_]*)".to_string());
    v.insert("num_dec_exp".to_string(), "(?:e[+-]?{{num_dec}})".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_preprocessor-macro-indirection_2".to_string(), ContextId { index: 6043 });
    v.insert("mnemonics-sse3".to_string(), ContextId { index: 6091 });
    v.insert("operators".to_string(), ContextId { index: 6101 });
    v.insert("pop-if-not-whitespace".to_string(), ContextId { index: 6103 });
    v.insert("preprocessor".to_string(), ContextId { index: 6105 });
    v.insert("#anon_preprocessor-macro-define_0".to_string(), ContextId { index: 6030 });
    v.insert("#anon_directives_6".to_string(), ContextId { index: 6025 });
    v.insert("labels".to_string(), ContextId { index: 6065 });
    v.insert("mnemonics-mmx".to_string(), ContextId { index: 6086 });
    v.insert("#anon_directives_5".to_string(), ContextId { index: 6024 });
    v.insert("prefixes".to_string(), ContextId { index: 6104 });
    v.insert("comments".to_string(), ContextId { index: 6058 });
    v.insert("mnemonics-future-intel-opmask".to_string(), ContextId { index: 6078 });
    v.insert("__start".to_string(), ContextId { index: 6057 });
    v.insert("mnemonics-sse".to_string(), ContextId { index: 6089 });
    v.insert("mnemonics-avx".to_string(), ContextId { index: 6072 });
    v.insert("#anon_directives_9".to_string(), ContextId { index: 6028 });
    v.insert("#anon_section-parameters_1".to_string(), ContextId { index: 6050 });
    v.insert("mnemonics-invalid".to_string(), ContextId { index: 6084 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 6013 });
    v.insert("mnemonics-pseudo-ops".to_string(), ContextId { index: 6087 });
    v.insert("#anon_directives_14".to_string(), ContextId { index: 6020 });
    v.insert("#anon_directives_12".to_string(), ContextId { index: 6018 });
    v.insert("mnemonics-vmx".to_string(), ContextId { index: 6099 });
    v.insert("preprocessor-conditions-constant".to_string(), ContextId { index: 6107 });
    v.insert("#anon_preprocessor-macro-indirection_1".to_string(), ContextId { index: 6042 });
    v.insert("preprocessor-macro-indirection".to_string(), ContextId { index: 6119 });
    v.insert("string-single".to_string(), ContextId { index: 6132 });
    v.insert("line-ending".to_string(), ContextId { index: 6067 });
    v.insert("#anon_preprocessor-macro-multiline_1".to_string(), ContextId { index: 6045 });
    v.insert("#anon_directives_10".to_string(), ContextId { index: 6016 });
    v.insert("preprocessor-conditions-content-block-common".to_string(), ContextId { index: 6109 });
    v.insert("mnemonics-undocumented".to_string(), ContextId { index: 6098 });
    v.insert("mnemonics-sse2".to_string(), ContextId { index: 6090 });
    v.insert("preprocessor-macro-definition".to_string(), ContextId { index: 6115 });
    v.insert("#anon_directives_2".to_string(), ContextId { index: 6021 });
    v.insert("mnemonics-intel-isa-sgx".to_string(), ContextId { index: 6082 });
    v.insert("string-double".to_string(), ContextId { index: 6131 });
    v.insert("#anon_directives_11".to_string(), ContextId { index: 6017 });
    v.insert("preprocessor-macro-params".to_string(), ContextId { index: 6122 });
    v.insert("#anon_directives_13".to_string(), ContextId { index: 6019 });
    v.insert("string-backquote".to_string(), ContextId { index: 6129 });
    v.insert("mnemonics-avx2".to_string(), ContextId { index: 6073 });
    v.insert("structs".to_string(), ContextId { index: 6134 });
    v.insert("data-types".to_string(), ContextId { index: 6059 });
    v.insert("pop".to_string(), ContextId { index: 6102 });
    v.insert("#anon_preprocessor-macro-define_2".to_string(), ContextId { index: 6032 });
    v.insert("mnemonics-intel-isa-sha".to_string(), ContextId { index: 6083 });
    v.insert("preprocessor-macro-parameter".to_string(), ContextId { index: 6121 });
    v.insert("prototype".to_string(), ContextId { index: 6125 });
    v.insert("line-continuation".to_string(), ContextId { index: 6066 });
    v.insert("#anon_structs_1".to_string(), ContextId { index: 6052 });
    v.insert("#anon_preprocessor-macro-define_7".to_string(), ContextId { index: 6037 });
    v.insert("#anon_directives_7".to_string(), ContextId { index: 6026 });
    v.insert("#anon_preprocessor-macro-params_0".to_string(), ContextId { index: 6046 });
    v.insert("numbers".to_string(), ContextId { index: 6100 });
    v.insert("#anon_structs_3".to_string(), ContextId { index: 6054 });
    v.insert("#anon_directives_8".to_string(), ContextId { index: 6027 });
    v.insert("mnemonics".to_string(), ContextId { index: 6069 });
    v.insert("#anon_preprocessor-macro-define_1".to_string(), ContextId { index: 6031 });
    v.insert("#anon_preprocessor-macro-define_3".to_string(), ContextId { index: 6033 });
    v.insert("floating-point".to_string(), ContextId { index: 6063 });
    v.insert("#anon_preprocessor-other_0".to_string(), ContextId { index: 6047 });
    v.insert("preprocessor-conditions-content-block-multiline".to_string(), ContextId { index: 6110 });
    v.insert("preprocessor-macro-definition-multiline-with-parameters".to_string(), ContextId { index: 6118 });
    v.insert("mnemonics-sse4".to_string(), ContextId { index: 6092 });
    v.insert("string-content".to_string(), ContextId { index: 6130 });
    v.insert("mnemonics-supplemental-amd".to_string(), ContextId { index: 6093 });
    v.insert("section-parameters".to_string(), ContextId { index: 6128 });
    v.insert("#anon_section-name_0".to_string(), ContextId { index: 6048 });
    v.insert("#anon_directives_1".to_string(), ContextId { index: 6015 });
    v.insert("mnemonics-future-intel-cet".to_string(), ContextId { index: 6077 });
    v.insert("#anon_section-parameters_0".to_string(), ContextId { index: 6049 });
    v.insert("mnemonics-future-intel-avx512".to_string(), ContextId { index: 6076 });
    v.insert("mnemonics-smx".to_string(), ContextId { index: 6088 });
    v.insert("__main".to_string(), ContextId { index: 6056 });
    v.insert("preprocessor-conditions-content-block".to_string(), ContextId { index: 6108 });
    v.insert("#anon_preprocessor-macro-define_5".to_string(), ContextId { index: 6035 });
    v.insert("main".to_string(), ContextId { index: 6068 });
    v.insert("mnemonics-supplemental-cyrix".to_string(), ContextId { index: 6094 });
    v.insert("preprocessor-conditions-parity".to_string(), ContextId { index: 6111 });
    v.insert("preprocessor-macro-definition-inside-multiline".to_string(), ContextId { index: 6116 });
    v.insert("mnemonics-intel-isa-mpx".to_string(), ContextId { index: 6081 });
    v.insert("preprocessor-macro-arguments-signature".to_string(), ContextId { index: 6112 });
    v.insert("mnemonics-system".to_string(), ContextId { index: 6096 });
    v.insert("registers".to_string(), ContextId { index: 6126 });
    v.insert("#anon_preprocessor-macro-define_4".to_string(), ContextId { index: 6034 });
    v.insert("#anon_structs_2".to_string(), ContextId { index: 6053 });
    v.insert("mnemonics-aesni".to_string(), ContextId { index: 6071 });
    v.insert("preprocessor-macro-definition-multiline".to_string(), ContextId { index: 6117 });
    v.insert("#anon_directives_0".to_string(), ContextId { index: 6014 });
    v.insert("#anon_preprocessor-macro-indirection_0".to_string(), ContextId { index: 6041 });
    v.insert("directives".to_string(), ContextId { index: 6060 });
    v.insert("#anon_preprocessor-macro-multiline_0".to_string(), ContextId { index: 6044 });
    v.insert("preprocessor-macro-params-illegal".to_string(), ContextId { index: 6123 });
    v.insert("#anon_directives_3".to_string(), ContextId { index: 6022 });
    v.insert("#anon_preprocessor-macro-define_6".to_string(), ContextId { index: 6036 });
    v.insert("#anon_preprocessor-macro-definition-multiline_0".to_string(), ContextId { index: 6038 });
    v.insert("mnemonics-supplemental-via".to_string(), ContextId { index: 6095 });
    v.insert("#anon_preprocessor-conditions-constant_0".to_string(), ContextId { index: 6029 });
    v.insert("preprocessor-macro-define".to_string(), ContextId { index: 6114 });
    v.insert("section-name".to_string(), ContextId { index: 6127 });
    v.insert("mnemonics-fpu".to_string(), ContextId { index: 6074 });
    v.insert("strings".to_string(), ContextId { index: 6133 });
    v.insert("mnemonics-future-intel".to_string(), ContextId { index: 6075 });
    v.insert("mnemonics-general-purpose".to_string(), ContextId { index: 6080 });
    v.insert("preprocessor-macro-conditions-multiline".to_string(), ContextId { index: 6113 });
    v.insert("preprocessor-other".to_string(), ContextId { index: 6124 });
    v.insert("mnemonics-64bit".to_string(), ContextId { index: 6070 });
    v.insert("mnemonics-future-intel-other".to_string(), ContextId { index: 6079 });
    v.insert("preprocessor-macro-multiline".to_string(), ContextId { index: 6120 });
    v.insert("mnemonics-invalid-amd-sse5".to_string(), ContextId { index: 6085 });
    v.insert("#anon_support_0".to_string(), ContextId { index: 6055 });
    v.insert("#anon_structs_0".to_string(), ContextId { index: 6051 });
    v.insert("#anon_directives_4".to_string(), ContextId { index: 6023 });
    v.insert("#anon_preprocessor-macro-definition-multiline_2".to_string(), ContextId { index: 6040 });
    v.insert("export-parameters".to_string(), ContextId { index: 6062 });
    v.insert("integers".to_string(), ContextId { index: 6064 });
    v.insert("preprocessor-conditions".to_string(), ContextId { index: 6106 });
    v.insert("support".to_string(), ContextId { index: 6135 });
    v.insert("entities".to_string(), ContextId { index: 6061 });
    v.insert("#anon_preprocessor-macro-definition-multiline_1".to_string(), ContextId { index: 6039 });
    v.insert("mnemonics-tsx".to_string(), ContextId { index: 6097 });
    v
  }
} }