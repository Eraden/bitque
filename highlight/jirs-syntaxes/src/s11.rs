
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Clojure".to_string(),
  file_extensions: vec!["clj".to_string(),"cljc".to_string(),"cljs".to_string(),"edn".to_string()],
  scope: Scope { a: 844489354641408, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("atom".to_string(), "[^{{non_symbol_chars}}]+".to_string());
    v.insert("evil_octal".to_string(), "[-+]?0\\d+N?(?=[{{non_symbol_chars}}])".to_string());
    v.insert("hex_integer".to_string(), "({{sign}})(0[Xx])\\h+(N?)(?=[{{non_number_chars}}])".to_string());
    v.insert("non_symbol_start_chars".to_string(), "{{non_symbol_chars}}\\d#\':".to_string());
    v.insert("exponent".to_string(), "(?:[eE]{{sign}}\\d+)".to_string());
    v.insert("symbol".to_string(), "(?:/|[^{{non_symbol_start_chars}}][^{{non_symbol_chars}}]*)".to_string());
    v.insert("keyword".to_string(), "(:):?[^:{{non_symbol_chars}}][^{{non_symbol_chars}}]*".to_string());
    v.insert("non_char_chars".to_string(), "{{non_symbol_chars}}#\'".to_string());
    v.insert("rational".to_string(), "({{sign}})\\d+(/)\\d+(?=[{{non_number_chars}}])".to_string());
    v.insert("non_number_chars".to_string(), "{{non_symbol_chars}}#\'".to_string());
    v.insert("other_integer".to_string(), "({{sign}})((?:[2-9]|[1-9]\\d+)[Rr])[0-9A-Za-z]+(?=[{{non_number_chars}}])".to_string());
    v.insert("constant".to_string(), "(?:nil|true|false)(?=[{{non_symbol_chars}}])".to_string());
    v.insert("dec_integer".to_string(), "({{sign}})\\d+(N?)(?=[{{non_number_chars}}])".to_string());
    v.insert("non_symbol_chars".to_string(), "\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\".to_string());
    v.insert("float".to_string(), "({{sign}})\\d+(?:(?:(\\.)\\d+{{exponent}}?|{{exponent}})(M)?|(M))(?=[{{non_number_chars}}])".to_string());
    v.insert("sign".to_string(), "[-+]?".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("pop-type-list-tail".to_string(), ContextId { index: 1073 });
    v.insert("pop-dispatch-tag-expr".to_string(), ContextId { index: 1062 });
    v.insert("pop-protocol-list-tail".to_string(), ContextId { index: 1069 });
    v.insert("pop-fn-list-tail".to_string(), ContextId { index: 1064 });
    v.insert("pop-list-head".to_string(), ContextId { index: 1067 });
    v.insert("#anon_match-expr_0".to_string(), ContextId { index: 1044 });
    v.insert("pop-dispatch-expr".to_string(), ContextId { index: 1061 });
    v.insert("set-normal-list-tail".to_string(), ContextId { index: 1074 });
    v.insert("pop-symbolic-value".to_string(), ContextId { index: 1072 });
    v.insert("pop-declare-interface-list-tail".to_string(), ContextId { index: 1057 });
    v.insert("#anon_pop-regexp-tail_0".to_string(), ContextId { index: 1048 });
    v.insert("main".to_string(), ContextId { index: 1051 });
    v.insert("pop-declare-protocol-list-tail".to_string(), ContextId { index: 1059 });
    v.insert("match-expr".to_string(), ContextId { index: 1053 });
    v.insert("pop-declare-list-tail".to_string(), ContextId { index: 1058 });
    v.insert("pop-invoke-list-tail".to_string(), ContextId { index: 1066 });
    v.insert("__main".to_string(), ContextId { index: 1049 });
    v.insert("__start".to_string(), ContextId { index: 1050 });
    v.insert("match-constant-set-normal-list-tail".to_string(), ContextId { index: 1052 });
    v.insert("pop-string-tail".to_string(), ContextId { index: 1071 });
    v.insert("#anon_pop-expr_0".to_string(), ContextId { index: 1046 });
    v.insert("match-symbol-implemented-or-extended".to_string(), ContextId { index: 1055 });
    v.insert("match-noise".to_string(), ContextId { index: 1054 });
    v.insert("pop-list-tail".to_string(), ContextId { index: 1068 });
    v.insert("pop-declare-type-list-tail".to_string(), ContextId { index: 1060 });
    v.insert("#anon_pop-expr_1".to_string(), ContextId { index: 1047 });
    v.insert("pop-expr".to_string(), ContextId { index: 1063 });
    v.insert("pop-regexp-tail".to_string(), ContextId { index: 1070 });
    v.insert("pop-interface-list-tail".to_string(), ContextId { index: 1065 });
    v.insert("#anon_match-expr_1".to_string(), ContextId { index: 1045 });
    v.insert("pop-declare-def-list-tail".to_string(), ContextId { index: 1056 });
    v
  }
} }