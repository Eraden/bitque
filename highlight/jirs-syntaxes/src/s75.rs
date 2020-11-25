
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Rust".to_string(),
  file_extensions: vec!["rs".to_string()],
  scope: Scope { a: 844734167777280, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), "(?:(?:[[:alpha:]][_[:alnum:]]*|_[_[:alnum:]]+)\\b)".to_string());
    v.insert("escaped_byte".to_string(), "\\\\(x\\h{2}|n|r|t|0|\"|\'|\\\\)".to_string());
    v.insert("int_suffixes".to_string(), "i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize".to_string());
    v.insert("support_type".to_string(), "\\b(Copy|Send|Sized|Sync|Drop|Fn|FnMut|FnOnce|Box|ToOwned|Clone|PartialEq|PartialOrd|Eq|Ord|AsRef|AsMut|Into|From|Default|Iterator|Extend|IntoIterator|DoubleEndedIterator|ExactSizeIterator|Option|Some|None|Result|Ok|Err|SliceConcatExt|String|ToString|Vec)\\b".to_string());
    v.insert("escaped_char".to_string(), "\\\\(x\\h{2}|n|r|t|0|\"|\'|\\\\|u\\{\\h{1,6}\\})".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("try-closure".to_string(), ContextId { index: 5086 });
    v.insert("#anon_struct-tuple_1".to_string(), ContextId { index: 5012 });
    v.insert("comments".to_string(), ContextId { index: 5034 });
    v.insert("#anon_block-comments_0".to_string(), ContextId { index: 4966 });
    v.insert("pop-immediately".to_string(), ContextId { index: 5073 });
    v.insert("#anon_struct-identifier_0".to_string(), ContextId { index: 5010 });
    v.insert("impl-definition".to_string(), ContextId { index: 5055 });
    v.insert("#anon_byte-string_0".to_string(), ContextId { index: 4968 });
    v.insert("#anon_statements_0".to_string(), ContextId { index: 5000 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 4976 });
    v.insert("#anon_statements_5".to_string(), ContextId { index: 5005 });
    v.insert("#anon_closure-body_0".to_string(), ContextId { index: 4971 });
    v.insert("fn-where".to_string(), ContextId { index: 5045 });
    v.insert("#anon_generic-angles-contents_3".to_string(), ContextId { index: 4987 });
    v.insert("return-type".to_string(), ContextId { index: 5077 });
    v.insert("impl-generic".to_string(), ContextId { index: 5057 });
    v.insert("impl-where".to_string(), ContextId { index: 5059 });
    v.insert("#anon_generic-angles-contents_0".to_string(), ContextId { index: 4984 });
    v.insert("fn-generic".to_string(), ContextId { index: 5042 });
    v.insert("attribute-call".to_string(), ContextId { index: 5019 });
    v.insert("#anon_macro-body_1".to_string(), ContextId { index: 4993 });
    v.insert("#anon_statements_2".to_string(), ContextId { index: 5002 });
    v.insert("closure-return".to_string(), ContextId { index: 5033 });
    v.insert("constant-integer-expression".to_string(), ContextId { index: 5035 });
    v.insert("#anon_macro-block_0".to_string(), ContextId { index: 4989 });
    v.insert("#anon_macro-matcher_1".to_string(), ContextId { index: 4996 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 5007 });
    v.insert("__main".to_string(), ContextId { index: 5016 });
    v.insert("closure".to_string(), ContextId { index: 5029 });
    v.insert("#anon_return-type_0".to_string(), ContextId { index: 4999 });
    v.insert("#anon_format-raw-string_0".to_string(), ContextId { index: 4982 });
    v.insert("attributes".to_string(), ContextId { index: 5020 });
    v.insert("#anon_fn-body_0".to_string(), ContextId { index: 4978 });
    v.insert("#anon_closure-parameters_2".to_string(), ContextId { index: 4974 });
    v.insert("#anon_struct-classic_1".to_string(), ContextId { index: 5009 });
    v.insert("#anon_statements_6".to_string(), ContextId { index: 5006 });
    v.insert("main".to_string(), ContextId { index: 5068 });
    v.insert("__start".to_string(), ContextId { index: 5017 });
    v.insert("block".to_string(), ContextId { index: 5022 });
    v.insert("struct-identifier".to_string(), ContextId { index: 5084 });
    v.insert("macro-metavariable".to_string(), ContextId { index: 5066 });
    v.insert("macro-match-operator".to_string(), ContextId { index: 5064 });
    v.insert("format-raw-string".to_string(), ContextId { index: 5047 });
    v.insert("#anon_type-any-identifier_2".to_string(), ContextId { index: 5015 });
    v.insert("block-comments".to_string(), ContextId { index: 5024 });
    v.insert("#anon_char_0".to_string(), ContextId { index: 4970 });
    v.insert("#anon_fn-parameters_1".to_string(), ContextId { index: 4980 });
    v.insert("closure-explicit-body".to_string(), ContextId { index: 5031 });
    v.insert("macro-terminator".to_string(), ContextId { index: 5067 });
    v.insert("#anon_type-any-identifier_1".to_string(), ContextId { index: 5014 });
    v.insert("#anon_raw-string_0".to_string(), ContextId { index: 4998 });
    v.insert("fn-return".to_string(), ContextId { index: 5044 });
    v.insert("block-body".to_string(), ContextId { index: 5023 });
    v.insert("#anon_impl-for_0".to_string(), ContextId { index: 4988 });
    v.insert("no-type-names".to_string(), ContextId { index: 5070 });
    v.insert("#anon_attributes_0".to_string(), ContextId { index: 4963 });
    v.insert("type-any-identifier".to_string(), ContextId { index: 5088 });
    v.insert("#anon_struct-classic_0".to_string(), ContextId { index: 5008 });
    v.insert("#anon_macro-body_2".to_string(), ContextId { index: 4994 });
    v.insert("#anon_statements_1".to_string(), ContextId { index: 5001 });
    v.insert("format-string".to_string(), ContextId { index: 5048 });
    v.insert("group-body".to_string(), ContextId { index: 5052 });
    v.insert("impl-for".to_string(), ContextId { index: 5056 });
    v.insert("#anon_attribute-call_0".to_string(), ContextId { index: 4962 });
    v.insert("integers".to_string(), ContextId { index: 5060 });
    v.insert("macro-block".to_string(), ContextId { index: 5061 });
    v.insert("byte".to_string(), ContextId { index: 5025 });
    v.insert("#anon_struct-tuple_0".to_string(), ContextId { index: 5011 });
    v.insert("prototype".to_string(), ContextId { index: 5074 });
    v.insert("type".to_string(), ContextId { index: 5087 });
    v.insert("#anon_basic-identifiers_0".to_string(), ContextId { index: 4965 });
    v.insert("generic-angles-contents".to_string(), ContextId { index: 5050 });
    v.insert("#anon_type-any-identifier_0".to_string(), ContextId { index: 5013 });
    v.insert("impl-body".to_string(), ContextId { index: 5054 });
    v.insert("else-pop".to_string(), ContextId { index: 5036 });
    v.insert("macro-block-contents".to_string(), ContextId { index: 5062 });
    v.insert("group".to_string(), ContextId { index: 5051 });
    v.insert("operators".to_string(), ContextId { index: 5072 });
    v.insert("format-escapes".to_string(), ContextId { index: 5046 });
    v.insert("numbers".to_string(), ContextId { index: 5071 });
    v.insert("macro-body".to_string(), ContextId { index: 5063 });
    v.insert("raw-byte-string".to_string(), ContextId { index: 5075 });
    v.insert("#anon_generic-angles-contents_1".to_string(), ContextId { index: 4985 });
    v.insert("after-operator".to_string(), ContextId { index: 5018 });
    v.insert("#anon_attributes_1".to_string(), ContextId { index: 4964 });
    v.insert("statements".to_string(), ContextId { index: 5078 });
    v.insert("char".to_string(), ContextId { index: 5027 });
    v.insert("#anon_macro-block_1".to_string(), ContextId { index: 4990 });
    v.insert("fn-body".to_string(), ContextId { index: 5040 });
    v.insert("fn-definition".to_string(), ContextId { index: 5041 });
    v.insert("#anon_macro-body_0".to_string(), ContextId { index: 4992 });
    v.insert("fn-parameters".to_string(), ContextId { index: 5043 });
    v.insert("escaped-char".to_string(), ContextId { index: 5038 });
    v.insert("escaped-byte".to_string(), ContextId { index: 5037 });
    v.insert("raw-string".to_string(), ContextId { index: 5076 });
    v.insert("#anon_macro-block_2".to_string(), ContextId { index: 4991 });
    v.insert("group-tail".to_string(), ContextId { index: 5053 });
    v.insert("#anon_fn-parameters_0".to_string(), ContextId { index: 4979 });
    v.insert("#anon_constant-integer-expression_0".to_string(), ContextId { index: 4977 });
    v.insert("#anon_format-string_0".to_string(), ContextId { index: 4983 });
    v.insert("struct-classic".to_string(), ContextId { index: 5083 });
    v.insert("#anon_byte_0".to_string(), ContextId { index: 4969 });
    v.insert("closure-parameters".to_string(), ContextId { index: 5032 });
    v.insert("macro-matcher".to_string(), ContextId { index: 5065 });
    v.insert("#anon_closure-parameters_0".to_string(), ContextId { index: 4972 });
    v.insert("#anon_generic-angles-contents_2".to_string(), ContextId { index: 4986 });
    v.insert("#anon_macro-matcher_0".to_string(), ContextId { index: 4995 });
    v.insert("#anon_raw-byte-string_0".to_string(), ContextId { index: 4997 });
    v.insert("basic-identifiers".to_string(), ContextId { index: 5021 });
    v.insert("#anon_block-comments_1".to_string(), ContextId { index: 4967 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 4975 });
    v.insert("#anon_fn-parameters_2".to_string(), ContextId { index: 4981 });
    v.insert("closure-body".to_string(), ContextId { index: 5030 });
    v.insert("statements-block".to_string(), ContextId { index: 5079 });
    v.insert("strings".to_string(), ContextId { index: 5081 });
    v.insert("no-path-identifiers".to_string(), ContextId { index: 5069 });
    v.insert("#anon_statements_3".to_string(), ContextId { index: 5003 });
    v.insert("#anon_closure-parameters_1".to_string(), ContextId { index: 4973 });
    v.insert("struct-body".to_string(), ContextId { index: 5082 });
    v.insert("struct-tuple".to_string(), ContextId { index: 5085 });
    v.insert("floats".to_string(), ContextId { index: 5039 });
    v.insert("generic-angles".to_string(), ContextId { index: 5049 });
    v.insert("string".to_string(), ContextId { index: 5080 });
    v.insert("#anon_statements_4".to_string(), ContextId { index: 5004 });
    v.insert("byte-string".to_string(), ContextId { index: 5026 });
    v.insert("impl-identifier".to_string(), ContextId { index: 5058 });
    v.insert("chars".to_string(), ContextId { index: 5028 });
    v
  }
} }