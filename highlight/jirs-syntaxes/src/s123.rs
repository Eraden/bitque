
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML (Twig)".to_string(),
  file_extensions: vec!["twig".to_string(),"html.twig".to_string()],
  scope: Scope { a: 281496459149312, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 7660 });
    v.insert("#anon_main_14".to_string(), ContextId { index: 7665 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 7666 });
    v.insert("__main".to_string(), ContextId { index: 7696 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 7668 });
    v.insert("entities".to_string(), ContextId { index: 7699 });
    v.insert("#anon_twig-arrays_0".to_string(), ContextId { index: 7684 });
    v.insert("tag-id-attribute".to_string(), ContextId { index: 7707 });
    v.insert("twig-filters".to_string(), ContextId { index: 7712 });
    v.insert("twig-keywords".to_string(), ContextId { index: 7719 });
    v.insert("#anon_twig-properties_0".to_string(), ContextId { index: 7692 });
    v.insert("#anon_twig-functions-warg_0".to_string(), ContextId { index: 7688 });
    v.insert("twig-arrays".to_string(), ContextId { index: 7709 });
    v.insert("#anon_tag-id-attribute_0".to_string(), ContextId { index: 7681 });
    v.insert("#anon_tag-id-attribute_1".to_string(), ContextId { index: 7682 });
    v.insert("twig-operators".to_string(), ContextId { index: 7722 });
    v.insert("#anon_twig-strings_1".to_string(), ContextId { index: 7695 });
    v.insert("twig-objects".to_string(), ContextId { index: 7721 });
    v.insert("#anon_main_9".to_string(), ContextId { index: 7673 });
    v.insert("#anon_main_13".to_string(), ContextId { index: 7664 });
    v.insert("#anon_twig-filters-warg-ud_0".to_string(), ContextId { index: 7686 });
    v.insert("tag-stuff".to_string(), ContextId { index: 7708 });
    v.insert("twig-functions-warg".to_string(), ContextId { index: 7717 });
    v.insert("#anon_twig-print-tag_0".to_string(), ContextId { index: 7691 });
    v.insert("tag-generic-attribute".to_string(), ContextId { index: 7706 });
    v.insert("twig-hashes".to_string(), ContextId { index: 7718 });
    v.insert("#anon_twig-hashes_0".to_string(), ContextId { index: 7689 });
    v.insert("embedded-code".to_string(), ContextId { index: 7698 });
    v.insert("#anon_ruby_2".to_string(), ContextId { index: 7678 });
    v.insert("#anon_twig-strings_0".to_string(), ContextId { index: 7694 });
    v.insert("#anon_main_11".to_string(), ContextId { index: 7662 });
    v.insert("#anon_tag-id-attribute_2".to_string(), ContextId { index: 7683 });
    v.insert("#anon_main_10".to_string(), ContextId { index: 7661 });
    v.insert("main".to_string(), ContextId { index: 7700 });
    v.insert("#anon_twig-comment-tag_0".to_string(), ContextId { index: 7685 });
    v.insert("twig-constants".to_string(), ContextId { index: 7711 });
    v.insert("#anon_ruby_1".to_string(), ContextId { index: 7677 });
    v.insert("#anon_string-single-quoted_0".to_string(), ContextId { index: 7680 });
    v.insert("string-double-quoted".to_string(), ContextId { index: 7704 });
    v.insert("#anon_python_0".to_string(), ContextId { index: 7675 });
    v.insert("twig-print-tag".to_string(), ContextId { index: 7723 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 7669 });
    v.insert("php".to_string(), ContextId { index: 7701 });
    v.insert("#anon_php_0".to_string(), ContextId { index: 7674 });
    v.insert("twig-filters-warg".to_string(), ContextId { index: 7714 });
    v.insert("twig-properties".to_string(), ContextId { index: 7724 });
    v.insert("twig-strings".to_string(), ContextId { index: 7726 });
    v.insert("twig-functions".to_string(), ContextId { index: 7716 });
    v.insert("twig-statement-tag".to_string(), ContextId { index: 7725 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 7670 });
    v.insert("#anon_twig-filters-warg_0".to_string(), ContextId { index: 7687 });
    v.insert("string-single-quoted".to_string(), ContextId { index: 7705 });
    v.insert("#anon_main_12".to_string(), ContextId { index: 7663 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 7672 });
    v.insert("#anon_twig-macros_0".to_string(), ContextId { index: 7690 });
    v.insert("ruby".to_string(), ContextId { index: 7703 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 7667 });
    v.insert("twig-filters-warg-ud".to_string(), ContextId { index: 7715 });
    v.insert("#anon_ruby_0".to_string(), ContextId { index: 7676 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 7671 });
    v.insert("twig-filters-ud".to_string(), ContextId { index: 7713 });
    v.insert("__start".to_string(), ContextId { index: 7697 });
    v.insert("twig-macros".to_string(), ContextId { index: 7720 });
    v.insert("python".to_string(), ContextId { index: 7702 });
    v.insert("#anon_twig-statement-tag_0".to_string(), ContextId { index: 7693 });
    v.insert("#anon_string-double-quoted_0".to_string(), ContextId { index: 7679 });
    v.insert("twig-comment-tag".to_string(), ContextId { index: 7710 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 7659 });
    v
  }
} }