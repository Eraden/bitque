
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Apache Conf".to_string(),
  file_extensions: vec!["envvars".to_string(),"htaccess".to_string(),"HTACCESS".to_string(),"htgroups".to_string(),"HTGROUPS".to_string(),"htpasswd".to_string(),"HTPASSWD".to_string(),".htaccess".to_string(),".HTACCESS".to_string(),".htgroups".to_string(),".HTGROUPS".to_string(),".htpasswd".to_string(),".HTPASSWD".to_string()],
  scope: Scope { a: 844781412417536, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 5913 });
    v.insert("__start".to_string(), ContextId { index: 5914 });
    v.insert("main".to_string(), ContextId { index: 5915 });
    v
  }
} }