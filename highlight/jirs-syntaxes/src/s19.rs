
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Common".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 281565172596736, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("pretty_formats_builtins".to_string(), "oneline|short|medium|fuller|full|email|raw".to_string());
    v.insert("pretty_formats_empty_value_modifiers".to_string(), "[-+ ]?".to_string());
    v.insert("comment_char".to_string(), "[#;]".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("email-domain".to_string(), ContextId { index: 1743 });
    v.insert("#anon_pretty-formats-colors_0".to_string(), ContextId { index: 1734 });
    v.insert("username".to_string(), ContextId { index: 1767 });
    v.insert("fnmatch-start".to_string(), ContextId { index: 1753 });
    v.insert("fnmatch-body".to_string(), ContextId { index: 1748 });
    v.insert("#anon_pretty-formats-wrap-formatting_0".to_string(), ContextId { index: 1737 });
    v.insert("line-continuation".to_string(), ContextId { index: 1757 });
    v.insert("pretty-formats-trailers".to_string(), ContextId { index: 1764 });
    v.insert("#anon_include-pretty-formats_0".to_string(), ContextId { index: 1731 });
    v.insert("pretty-formats-wrap-formatting".to_string(), ContextId { index: 1765 });
    v.insert("#anon_pretty-formats-trailers_0".to_string(), ContextId { index: 1735 });
    v.insert("#anon_pretty-formats-trailers_1".to_string(), ContextId { index: 1736 });
    v.insert("email-meta".to_string(), ContextId { index: 1745 });
    v.insert("pretty-formats-as-arg-minimal".to_string(), ContextId { index: 1761 });
    v.insert("email-name".to_string(), ContextId { index: 1746 });
    v.insert("eol-pop".to_string(), ContextId { index: 1747 });
    v.insert("comments".to_string(), ContextId { index: 1740 });
    v.insert("fnmatch-char-class-body".to_string(), ContextId { index: 1749 });
    v.insert("pretty-formats-string-formatting".to_string(), ContextId { index: 1763 });
    v.insert("#anon_line-continuation_0".to_string(), ContextId { index: 1733 });
    v.insert("fnmatch-char-class-operator".to_string(), ContextId { index: 1750 });
    v.insert("illegal-eol-pop".to_string(), ContextId { index: 1754 });
    v.insert("pretty-formats".to_string(), ContextId { index: 1759 });
    v.insert("comments-line".to_string(), ContextId { index: 1741 });
    v.insert("references".to_string(), ContextId { index: 1766 });
    v.insert("pretty-formats-colors".to_string(), ContextId { index: 1762 });
    v.insert("email".to_string(), ContextId { index: 1742 });
    v.insert("email-end".to_string(), ContextId { index: 1744 });
    v.insert("pretty-formats-as-arg".to_string(), ContextId { index: 1760 });
    v.insert("__main".to_string(), ContextId { index: 1738 });
    v.insert("fnmatch-char-class-start".to_string(), ContextId { index: 1751 });
    v.insert("include-pretty-formats".to_string(), ContextId { index: 1755 });
    v.insert("fnmatch-dir-pattern".to_string(), ContextId { index: 1752 });
    v.insert("issue".to_string(), ContextId { index: 1756 });
    v.insert("main".to_string(), ContextId { index: 1758 });
    v.insert("__start".to_string(), ContextId { index: 1739 });
    v.insert("#anon_include-pretty-formats_1".to_string(), ContextId { index: 1732 });
    v
  }
} }