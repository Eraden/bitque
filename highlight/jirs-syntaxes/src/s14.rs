
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Diff".to_string(),
  file_extensions: vec!["diff".to_string(),"patch".to_string()],
  scope: Scope { a: 844506534510592, b: 0 },
  first_line_match: Some("(?x)^\n    (===\\ modified\\ file\n    |==== \\s* // .+ \\s - \\s .+ \\s+ ====\n    |Index:[ ]\n    |---\\ [^%]\n    |\\*\\*\\*.*\\d{4}\\s*$\n    |\\d+(,\\d+)* (a|d|c) \\d+(,\\d+)* $\n    |diff\\ --git[ ]\n    )".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 1372 });
    v.insert("main".to_string(), ContextId { index: 1373 });
    v.insert("__main".to_string(), ContextId { index: 1371 });
    v
  }
} }