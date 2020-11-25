
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "AWK".to_string(),
  file_extensions: vec!["awk".to_string()],
  scope: Scope { a: 844777117450240, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("NAME".to_string(), "[A-Za-z_][A-Za-z_0-9]*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_regexp_1".to_string(), ContextId { index: 5889 });
    v.insert("#anon_infix-operator_0".to_string(), ContextId { index: 5886 });
    v.insert("variable".to_string(), ContextId { index: 5912 });
    v.insert("function".to_string(), ContextId { index: 5900 });
    v.insert("function-definition".to_string(), ContextId { index: 5901 });
    v.insert("#anon_regexp_0".to_string(), ContextId { index: 5888 });
    v.insert("groupings".to_string(), ContextId { index: 5902 });
    v.insert("comment".to_string(), ContextId { index: 5896 });
    v.insert("#anon_function-definition_0".to_string(), ContextId { index: 5885 });
    v.insert("escaped-char".to_string(), ContextId { index: 5898 });
    v.insert("infix-operator".to_string(), ContextId { index: 5903 });
    v.insert("main".to_string(), ContextId { index: 5905 });
    v.insert("#anon_regexp_2".to_string(), ContextId { index: 5890 });
    v.insert("numeric-constant".to_string(), ContextId { index: 5906 });
    v.insert("constant".to_string(), ContextId { index: 5897 });
    v.insert("command".to_string(), ContextId { index: 5895 });
    v.insert("#anon_procedure_0".to_string(), ContextId { index: 5887 });
    v.insert("__main".to_string(), ContextId { index: 5892 });
    v.insert("__start".to_string(), ContextId { index: 5893 });
    v.insert("pattern".to_string(), ContextId { index: 5907 });
    v.insert("prefix-operator".to_string(), ContextId { index: 5908 });
    v.insert("expression".to_string(), ContextId { index: 5899 });
    v.insert("regexp".to_string(), ContextId { index: 5910 });
    v.insert("procedure".to_string(), ContextId { index: 5909 });
    v.insert("string-constant".to_string(), ContextId { index: 5911 });
    v.insert("builtin-pattern".to_string(), ContextId { index: 5894 });
    v.insert("#anon_string-constant_0".to_string(), ContextId { index: 5891 });
    v.insert("keyword".to_string(), ContextId { index: 5904 });
    v
  }
} }