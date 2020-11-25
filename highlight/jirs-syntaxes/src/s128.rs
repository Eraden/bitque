
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Jinja2".to_string(),
  file_extensions: vec!["j2".to_string(),"jinja2".to_string(),"jinja".to_string()],
  scope: Scope { a: 844936031240192, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 7874 });
    v.insert("string".to_string(), ContextId { index: 7876 });
    v.insert("#anon_expression_2".to_string(), ContextId { index: 7861 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 7867 });
    v.insert("__main".to_string(), ContextId { index: 7869 });
    v.insert("escaped_unicode_char".to_string(), ContextId { index: 7872 });
    v.insert("simple_escapes".to_string(), ContextId { index: 7875 });
    v.insert("#anon_expression_5".to_string(), ContextId { index: 7864 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 7866 });
    v.insert("#anon_expression_4".to_string(), ContextId { index: 7863 });
    v.insert("#anon_expression_1".to_string(), ContextId { index: 7860 });
    v.insert("expression".to_string(), ContextId { index: 7873 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 7865 });
    v.insert("__start".to_string(), ContextId { index: 7870 });
    v.insert("#anon_expression_3".to_string(), ContextId { index: 7862 });
    v.insert("escaped_char".to_string(), ContextId { index: 7871 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 7868 });
    v.insert("#anon_expression_0".to_string(), ContextId { index: 7859 });
    v
  }
} }