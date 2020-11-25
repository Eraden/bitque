
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Rd (R Documentation)".to_string(),
  file_extensions: vec!["rd".to_string()],
  scope: Scope { a: 281629597958209, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_deqn_0".to_string(), ContextId { index: 4679 });
    v.insert("#anon_eqn_0".to_string(), ContextId { index: 4680 });
    v.insert("#anon_r-code_0".to_string(), ContextId { index: 4682 });
    v.insert("#anon_r-code_1".to_string(), ContextId { index: 4683 });
    v.insert("__main".to_string(), ContextId { index: 4688 });
    v.insert("#anon_rd-stucture_1".to_string(), ContextId { index: 4687 });
    v.insert("#anon_global-braces_0".to_string(), ContextId { index: 4681 });
    v.insert("eqn".to_string(), ContextId { index: 4691 });
    v.insert("#anon_r-code_3".to_string(), ContextId { index: 4685 });
    v.insert("global-braces".to_string(), ContextId { index: 4692 });
    v.insert("#anon_rd-stucture_0".to_string(), ContextId { index: 4686 });
    v.insert("latex-equations".to_string(), ContextId { index: 4693 });
    v.insert("prototype".to_string(), ContextId { index: 4695 });
    v.insert("deqn".to_string(), ContextId { index: 4690 });
    v.insert("__start".to_string(), ContextId { index: 4689 });
    v.insert("rd-stucture".to_string(), ContextId { index: 4697 });
    v.insert("#anon_r-code_2".to_string(), ContextId { index: 4684 });
    v.insert("main".to_string(), ContextId { index: 4694 });
    v.insert("r-code".to_string(), ContextId { index: 4696 });
    v
  }
} }