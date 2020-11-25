
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "CoffeeScript".to_string(),
  file_extensions: vec!["coffee".to_string(),"Cakefile".to_string(),"coffee.erb".to_string(),"cson".to_string()],
  scope: Scope { a: 844820067123200, b: 0 },
  first_line_match: Some("^#!.*\\bcoffee".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_4".to_string(), ContextId { index: 6587 });
    v.insert("main".to_string(), ContextId { index: 6597 });
    v.insert("#anon_interpolated_coffee_0".to_string(), ContextId { index: 6582 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 6586 });
    v.insert("numeric".to_string(), ContextId { index: 6598 });
    v.insert("interpolated_coffee".to_string(), ContextId { index: 6596 });
    v.insert("__start".to_string(), ContextId { index: 6592 });
    v.insert("single_quoted_string".to_string(), ContextId { index: 6599 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6583 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6584 });
    v.insert("__main".to_string(), ContextId { index: 6591 });
    v.insert("#anon_double_quoted_string_0".to_string(), ContextId { index: 6581 });
    v.insert("#anon_single_quoted_string_0".to_string(), ContextId { index: 6590 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 6585 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 6589 });
    v.insert("variable_name".to_string(), ContextId { index: 6600 });
    v.insert("double_quoted_string".to_string(), ContextId { index: 6593 });
    v.insert("instance_variable".to_string(), ContextId { index: 6595 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 6588 });
    v.insert("embedded_comment".to_string(), ContextId { index: 6594 });
    v
  }
} }