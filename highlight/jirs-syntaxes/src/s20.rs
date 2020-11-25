
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Config".to_string(),
  file_extensions: vec!["gitconfig".to_string(),".gitconfig".to_string(),".gitmodules".to_string()],
  scope: Scope { a: 281565172662272, b: 0 },
  first_line_match: Some("^\\[core\\]".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("zero_to_255".to_string(), "25[0-5]|2[0-4][0-9]|1\\d\\d|[1-9][0-9]|[0-9]".to_string());
    v.insert("variable_name".to_string(), "[a-zA-Z][\\w-]*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("other-value".to_string(), ContextId { index: 1791 });
    v.insert("section-header".to_string(), ContextId { index: 1793 });
    v.insert("#anon_line-continuation_0".to_string(), ContextId { index: 1770 });
    v.insert("prototype".to_string(), ContextId { index: 1792 });
    v.insert("#anon_string-unquoted_0".to_string(), ContextId { index: 1776 });
    v.insert("#anon_shell-script_3".to_string(), ContextId { index: 1774 });
    v.insert("line-continuation".to_string(), ContextId { index: 1788 });
    v.insert("illegal-line-end-pop".to_string(), ContextId { index: 1785 });
    v.insert("string-unquoted".to_string(), ContextId { index: 1798 });
    v.insert("escape".to_string(), ContextId { index: 1781 });
    v.insert("expect-section".to_string(), ContextId { index: 1783 });
    v.insert("illegal-line-end".to_string(), ContextId { index: 1784 });
    v.insert("#anon_shell-script_1".to_string(), ContextId { index: 1772 });
    v.insert("line-end".to_string(), ContextId { index: 1789 });
    v.insert("section-name".to_string(), ContextId { index: 1795 });
    v.insert("string-quoted".to_string(), ContextId { index: 1797 });
    v.insert("#anon_key-color-pair_0".to_string(), ContextId { index: 1768 });
    v.insert("expect-line-end".to_string(), ContextId { index: 1782 });
    v.insert("subsection-name-unquoted".to_string(), ContextId { index: 1800 });
    v.insert("__main".to_string(), ContextId { index: 1777 });
    v.insert("key-value-pair".to_string(), ContextId { index: 1787 });
    v.insert("shell-script".to_string(), ContextId { index: 1796 });
    v.insert("#anon_string-quoted_0".to_string(), ContextId { index: 1775 });
    v.insert("section-header-end".to_string(), ContextId { index: 1794 });
    v.insert("__start".to_string(), ContextId { index: 1778 });
    v.insert("#anon_shell-script_0".to_string(), ContextId { index: 1771 });
    v.insert("#anon_shell-script_2".to_string(), ContextId { index: 1773 });
    v.insert("boolean".to_string(), ContextId { index: 1779 });
    v.insert("key-color-pair".to_string(), ContextId { index: 1786 });
    v.insert("main".to_string(), ContextId { index: 1790 });
    v.insert("color-value".to_string(), ContextId { index: 1780 });
    v.insert("#anon_key-value-pair_0".to_string(), ContextId { index: 1769 });
    v.insert("subsection-name-quoted".to_string(), ContextId { index: 1799 });
    v
  }
} }