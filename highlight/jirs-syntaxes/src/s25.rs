
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Rebase Todo".to_string(),
  file_extensions: vec!["git-rebase-todo".to_string()],
  scope: Scope { a: 281565172989952, b: 0 },
  first_line_match: Some("^(?:drop|edit|exec|fixup|pick|reword|squash|[defprsx]) \\h{7,} ".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("comment_char".to_string(), "[#;]".to_string());
    v.insert("hash".to_string(), "\\b\\h{7,}\\b".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("commit-subject".to_string(), ContextId { index: 1835 });
    v.insert("heading".to_string(), ContextId { index: 1836 });
    v.insert("line-end".to_string(), ContextId { index: 1837 });
    v.insert("main".to_string(), ContextId { index: 1838 });
    v.insert("rebase-msg".to_string(), ContextId { index: 1839 });
    v.insert("commands-help".to_string(), ContextId { index: 1830 });
    v.insert("commit-hash".to_string(), ContextId { index: 1834 });
    v.insert("__start".to_string(), ContextId { index: 1829 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 1826 });
    v.insert("#anon_commit-subject_0".to_string(), ContextId { index: 1827 });
    v.insert("comments".to_string(), ContextId { index: 1831 });
    v.insert("commit".to_string(), ContextId { index: 1832 });
    v.insert("commit-commands".to_string(), ContextId { index: 1833 });
    v.insert("__main".to_string(), ContextId { index: 1828 });
    v
  }
} }