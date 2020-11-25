
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Attributes".to_string(),
  file_extensions: vec!["attributes".to_string(),"gitattributes".to_string(),".gitattributes".to_string()],
  scope: Scope { a: 281565172465664, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("meta-encoding".to_string(), ContextId { index: 1700 });
    v.insert("#anon_attribute-value-content_0".to_string(), ContextId { index: 1686 });
    v.insert("attributes-list".to_string(), ContextId { index: 1695 });
    v.insert("main".to_string(), ContextId { index: 1698 });
    v.insert("pattern-attributes".to_string(), ContextId { index: 1708 });
    v.insert("meta-text".to_string(), ContextId { index: 1705 });
    v.insert("meta-filter".to_string(), ContextId { index: 1702 });
    v.insert("meta-other".to_string(), ContextId { index: 1704 });
    v.insert("__start".to_string(), ContextId { index: 1688 });
    v.insert("attribute-separator".to_string(), ContextId { index: 1691 });
    v.insert("macro".to_string(), ContextId { index: 1697 });
    v.insert("meta-diff".to_string(), ContextId { index: 1699 });
    v.insert("attribute-value-expected".to_string(), ContextId { index: 1693 });
    v.insert("meta-eol".to_string(), ContextId { index: 1701 });
    v.insert("pattern-content".to_string(), ContextId { index: 1709 });
    v.insert("attribute-key".to_string(), ContextId { index: 1689 });
    v.insert("meta-merge".to_string(), ContextId { index: 1703 });
    v.insert("pattern-content-quoted".to_string(), ContextId { index: 1710 });
    v.insert("__main".to_string(), ContextId { index: 1687 });
    v.insert("pattern-content-unquoted".to_string(), ContextId { index: 1711 });
    v.insert("attribute-value-content".to_string(), ContextId { index: 1692 });
    v.insert("attribute-maybe-value-list".to_string(), ContextId { index: 1690 });
    v.insert("attribute-value-list".to_string(), ContextId { index: 1694 });
    v.insert("meta-whitespace".to_string(), ContextId { index: 1706 });
    v.insert("pattern".to_string(), ContextId { index: 1707 });
    v.insert("string-end".to_string(), ContextId { index: 1712 });
    v.insert("else-pop".to_string(), ContextId { index: 1696 });
    v
  }
} }