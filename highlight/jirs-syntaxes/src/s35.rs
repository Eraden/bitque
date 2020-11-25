
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Javadoc".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 281496454234112, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("id".to_string(), "(?:[\\p{L}_$][\\p{L}\\p{N}_$]*)".to_string());
    v.insert("javadoc_block_tag_terminator".to_string(), "(?=^\\s*\\*?\\s*@)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_javadoc-inline-tags_0".to_string(), ContextId { index: 2402 });
    v.insert("main".to_string(), ContextId { index: 2420 });
    v.insert("param-tag-parameter".to_string(), ContextId { index: 2421 });
    v.insert("#anon_contents_2".to_string(), ContextId { index: 2399 });
    v.insert("#anon_reference_1".to_string(), ContextId { index: 2408 });
    v.insert("link-tag-label".to_string(), ContextId { index: 2419 });
    v.insert("javadoc-inline-tag-terminator".to_string(), ContextId { index: 2417 });
    v.insert("reference".to_string(), ContextId { index: 2423 });
    v.insert("#anon_link-tag-label_0".to_string(), ContextId { index: 2403 });
    v.insert("javadoc-inline-tag-base".to_string(), ContextId { index: 2416 });
    v.insert("#anon_contents_0".to_string(), ContextId { index: 2397 });
    v.insert("javadoc-block-tags".to_string(), ContextId { index: 2415 });
    v.insert("contents".to_string(), ContextId { index: 2412 });
    v.insert("#anon_reference-in-inline-tag_0".to_string(), ContextId { index: 2406 });
    v.insert("__start".to_string(), ContextId { index: 2410 });
    v.insert("inline-formatting".to_string(), ContextId { index: 2413 });
    v.insert("__main".to_string(), ContextId { index: 2409 });
    v.insert("#anon_code-tag-bracket-balancing_0".to_string(), ContextId { index: 2396 });
    v.insert("#anon_contents_4".to_string(), ContextId { index: 2401 });
    v.insert("#anon_reference_0".to_string(), ContextId { index: 2407 });
    v.insert("code-tag-bracket-balancing".to_string(), ContextId { index: 2411 });
    v.insert("prototype".to_string(), ContextId { index: 2422 });
    v.insert("reference-in-inline-tag".to_string(), ContextId { index: 2424 });
    v.insert("#anon_contents_1".to_string(), ContextId { index: 2398 });
    v.insert("javadoc-block-tag-base".to_string(), ContextId { index: 2414 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 2405 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 2404 });
    v.insert("#anon_contents_3".to_string(), ContextId { index: 2400 });
    v.insert("javadoc-inline-tags".to_string(), ContextId { index: 2418 });
    v.insert("see-tag-content".to_string(), ContextId { index: 2425 });
    v
  }
} }