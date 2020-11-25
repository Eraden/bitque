
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Regular Expression".to_string(),
  file_extensions: vec!["re".to_string()],
  scope: Scope { a: 844613908692992, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("char_class".to_string(), "\\\\(?:[wWsSdDhHvVXR]|[pP](?:\\{[a-zA-Z_]+\\}|(L&|[A-Z][a-z]?)))".to_string());
    v.insert("char_escape".to_string(), "\\\\.".to_string());
    v.insert("lazy_or_possessive".to_string(), "[?+]?".to_string());
    v.insert("character_quantifier".to_string(), "[?*+]".to_string());
    v.insert("invalid_char_escape".to_string(), "\\\\[xcCM]".to_string());
    v.insert("ranged_quantifier".to_string(), "\\{\\d+(,\\d*)?\\}".to_string());
    v.insert("known_char_escape".to_string(), "\\\\(?:[tnrfae]|[0-7]{3}|x\\{\\h{1,7}\\}|x\\h\\h|c\\d+)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("base".to_string(), ContextId { index: 4749 });
    v.insert("base-literal".to_string(), ContextId { index: 4754 });
    v.insert("#anon_group-comment_0".to_string(), ContextId { index: 4744 });
    v.insert("__main".to_string(), ContextId { index: 4745 });
    v.insert("base-group".to_string(), ContextId { index: 4752 });
    v.insert("charset".to_string(), ContextId { index: 4757 });
    v.insert("backtracking-control-verb".to_string(), ContextId { index: 4748 });
    v.insert("#anon_charset_2".to_string(), ContextId { index: 4742 });
    v.insert("main".to_string(), ContextId { index: 4769 });
    v.insert("group-start-common".to_string(), ContextId { index: 4766 });
    v.insert("group-body".to_string(), ContextId { index: 4761 });
    v.insert("base-extended".to_string(), ContextId { index: 4751 });
    v.insert("special-escaped-char".to_string(), ContextId { index: 4772 });
    v.insert("group-start".to_string(), ContextId { index: 4765 });
    v.insert("base-group-extended".to_string(), ContextId { index: 4753 });
    v.insert("#anon_extended-patterns_0".to_string(), ContextId { index: 4743 });
    v.insert("character-class".to_string(), ContextId { index: 4756 });
    v.insert("group-comment".to_string(), ContextId { index: 4763 });
    v.insert("extended-patterns".to_string(), ContextId { index: 4759 });
    v.insert("group-extended".to_string(), ContextId { index: 4764 });
    v.insert("operators".to_string(), ContextId { index: 4770 });
    v.insert("backslashes".to_string(), ContextId { index: 4747 });
    v.insert("__start".to_string(), ContextId { index: 4746 });
    v.insert("base-common".to_string(), ContextId { index: 4750 });
    v.insert("base-literal-extended".to_string(), ContextId { index: 4755 });
    v.insert("escaped-char".to_string(), ContextId { index: 4758 });
    v.insert("group-body-extended".to_string(), ContextId { index: 4762 });
    v.insert("#anon_charset_1".to_string(), ContextId { index: 4741 });
    v.insert("literal".to_string(), ContextId { index: 4768 });
    v.insert("group-start-extended".to_string(), ContextId { index: 4767 });
    v.insert("quantifiers".to_string(), ContextId { index: 4771 });
    v.insert("unexpected-quantifier".to_string(), ContextId { index: 4773 });
    v.insert("unexpected-quantifier-pop".to_string(), ContextId { index: 4774 });
    v.insert("group".to_string(), ContextId { index: 4760 });
    v.insert("#anon_charset_0".to_string(), ContextId { index: 4740 });
    v
  }
} }