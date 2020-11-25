
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "AsciiDoc (Asciidoctor)".to_string(),
  file_extensions: vec!["adoc".to_string(),"ad".to_string(),"asciidoc".to_string()],
  scope: Scope { a: 281835753963520, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_block_quote_0".to_string(), ContextId { index: 5923 });
    v.insert("#anon_strong_0".to_string(), ContextId { index: 5935 });
    v.insert("__main".to_string(), ContextId { index: 5939 });
    v.insert("list_continuation".to_string(), ContextId { index: 5975 });
    v.insert("section_template".to_string(), ContextId { index: 5987 });
    v.insert("title_level_5".to_string(), ContextId { index: 5998 });
    v.insert("#anon_block_listing_0".to_string(), ContextId { index: 5919 });
    v.insert("block_sidebar".to_string(), ContextId { index: 5955 });
    v.insert("block_literal".to_string(), ContextId { index: 5950 });
    v.insert("inline_break".to_string(), ContextId { index: 5971 });
    v.insert("#anon_emphasis_double_0".to_string(), ContextId { index: 5928 });
    v.insert("inline".to_string(), ContextId { index: 5970 });
    v.insert("title_level_3".to_string(), ContextId { index: 5996 });
    v.insert("title_level_4".to_string(), ContextId { index: 5997 });
    v.insert("title_level_0".to_string(), ContextId { index: 5993 });
    v.insert("__start".to_string(), ContextId { index: 5940 });
    v.insert("attribute_list_line".to_string(), ContextId { index: 5942 });
    v.insert("#anon_macro_pass_0".to_string(), ContextId { index: 5929 });
    v.insert("blocks".to_string(), ContextId { index: 5959 });
    v.insert("macro_pass".to_string(), ContextId { index: 5978 });
    v.insert("inline_comment".to_string(), ContextId { index: 5973 });
    v.insert("#anon_block_comment_0".to_string(), ContextId { index: 5917 });
    v.insert("title_level_2".to_string(), ContextId { index: 5995 });
    v.insert("#anon_monospaced_0".to_string(), ContextId { index: 5932 });
    v.insert("block_id".to_string(), ContextId { index: 5948 });
    v.insert("#anon_block_open_0".to_string(), ContextId { index: 5921 });
    v.insert("block_pass".to_string(), ContextId { index: 5953 });
    v.insert("strong".to_string(), ContextId { index: 5989 });
    v.insert("#anon_block_source_fenced_0".to_string(), ContextId { index: 5925 });
    v.insert("#anon_mark_double_0".to_string(), ContextId { index: 5931 });
    v.insert("emphasis".to_string(), ContextId { index: 5963 });
    v.insert("#anon_block_pass_0".to_string(), ContextId { index: 5922 });
    v.insert("replacement".to_string(), ContextId { index: 5986 });
    v.insert("lines".to_string(), ContextId { index: 5974 });
    v.insert("olist_item_marker".to_string(), ContextId { index: 5984 });
    v.insert("attribute_reference".to_string(), ContextId { index: 5943 });
    v.insert("characters".to_string(), ContextId { index: 5960 });
    v.insert("block_title".to_string(), ContextId { index: 5958 });
    v.insert("lists".to_string(), ContextId { index: 5976 });
    v.insert("title_level_1".to_string(), ContextId { index: 5994 });
    v.insert("#anon_block_literal_0".to_string(), ContextId { index: 5920 });
    v.insert("entity_number".to_string(), ContextId { index: 5966 });
    v.insert("#anon_superscript_0".to_string(), ContextId { index: 5938 });
    v.insert("biblio_anchor".to_string(), ContextId { index: 5944 });
    v.insert("block_example".to_string(), ContextId { index: 5947 });
    v.insert("#anon_subscript_0".to_string(), ContextId { index: 5937 });
    v.insert("block_admonition_label".to_string(), ContextId { index: 5945 });
    v.insert("#anon_block_sidebar_0".to_string(), ContextId { index: 5924 });
    v.insert("block_quote".to_string(), ContextId { index: 5954 });
    v.insert("monospaced".to_string(), ContextId { index: 5982 });
    v.insert("#anon_strong_double_0".to_string(), ContextId { index: 5936 });
    v.insert("#anon_emphasis_0".to_string(), ContextId { index: 5927 });
    v.insert("block_listing".to_string(), ContextId { index: 5949 });
    v.insert("ulist_item_marker".to_string(), ContextId { index: 5999 });
    v.insert("block_source_fenced".to_string(), ContextId { index: 5956 });
    v.insert("strong_double".to_string(), ContextId { index: 5990 });
    v.insert("block_open".to_string(), ContextId { index: 5951 });
    v.insert("indexterm_double".to_string(), ContextId { index: 5968 });
    v.insert("#anon_monospaced_double_0".to_string(), ContextId { index: 5933 });
    v.insert("#anon_mark_0".to_string(), ContextId { index: 5930 });
    v.insert("block_page_break".to_string(), ContextId { index: 5952 });
    v.insert("emphasis_double".to_string(), ContextId { index: 5964 });
    v.insert("subscript".to_string(), ContextId { index: 5991 });
    v.insert("block_thematic_break".to_string(), ContextId { index: 5957 });
    v.insert("entity_name".to_string(), ContextId { index: 5965 });
    v.insert("indexterm_triple".to_string(), ContextId { index: 5969 });
    v.insert("#anon_dlist_item_label_0".to_string(), ContextId { index: 5926 });
    v.insert("main".to_string(), ContextId { index: 5979 });
    v.insert("section_titles".to_string(), ContextId { index: 5988 });
    v.insert("#anon_passthrough_0".to_string(), ContextId { index: 5934 });
    v.insert("#anon_block_example_0".to_string(), ContextId { index: 5918 });
    v.insert("attribute_entry".to_string(), ContextId { index: 5941 });
    v.insert("mark_double".to_string(), ContextId { index: 5981 });
    v.insert("monospaced_double".to_string(), ContextId { index: 5983 });
    v.insert("inline_callout".to_string(), ContextId { index: 5972 });
    v.insert("dlist_item_label".to_string(), ContextId { index: 5962 });
    v.insert("macro".to_string(), ContextId { index: 5977 });
    v.insert("escape".to_string(), ContextId { index: 5967 });
    v.insert("mark".to_string(), ContextId { index: 5980 });
    v.insert("colist_item_marker".to_string(), ContextId { index: 5961 });
    v.insert("superscript".to_string(), ContextId { index: 5992 });
    v.insert("block_comment".to_string(), ContextId { index: 5946 });
    v.insert("passthrough".to_string(), ContextId { index: 5985 });
    v.insert("#anon_attribute_entry_0".to_string(), ContextId { index: 5916 });
    v.insert("xref".to_string(), ContextId { index: 6000 });
    v
  }
} }