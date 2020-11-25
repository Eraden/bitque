
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "fstab".to_string(),
  file_extensions: vec!["fstab".to_string(),"crypttab".to_string()],
  scope: Scope { a: 844905966469120, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("clear".to_string(), ContextId { index: 7444 });
    v.insert("fstab_dump".to_string(), ContextId { index: 7448 });
    v.insert("fstab_pass".to_string(), ContextId { index: 7452 });
    v.insert("fstab_filesystem".to_string(), ContextId { index: 7449 });
    v.insert("fstab_options".to_string(), ContextId { index: 7451 });
    v.insert("__start".to_string(), ContextId { index: 7443 });
    v.insert("expected_eol".to_string(), ContextId { index: 7446 });
    v.insert("fstab_device".to_string(), ContextId { index: 7447 });
    v.insert("__main".to_string(), ContextId { index: 7442 });
    v.insert("comment".to_string(), ContextId { index: 7445 });
    v.insert("main".to_string(), ContextId { index: 7453 });
    v.insert("fstab_mountpoint".to_string(), ContextId { index: 7450 });
    v
  }
} }