
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "DotENV".to_string(),
  file_extensions: vec![".env".to_string(),".envrc".to_string(),".env.dist".to_string(),".env.local".to_string(),".env.sample".to_string(),".env.example".to_string(),".env.test".to_string(),".env.test.local".to_string(),".env.testing".to_string(),".env.dev".to_string(),".env.development".to_string(),".env.development.local".to_string(),".env.prod".to_string(),".env.production".to_string(),".env.production.local".to_string(),".env.dusk.local".to_string(),".flaskenv".to_string(),".env.staging".to_string()],
  scope: Scope { a: 844845836926976, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 6745 });
    v.insert("interpolation".to_string(), ContextId { index: 6749 });
    v.insert("#anon_interpolation_0".to_string(), ContextId { index: 6743 });
    v.insert("__main".to_string(), ContextId { index: 6746 });
    v.insert("__start".to_string(), ContextId { index: 6747 });
    v.insert("main".to_string(), ContextId { index: 6750 });
    v.insert("escape-characters".to_string(), ContextId { index: 6748 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6744 });
    v.insert("variable".to_string(), ContextId { index: 6751 });
    v
  }
} }