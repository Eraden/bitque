
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "PowerShell".to_string(),
  file_extensions: vec!["ps1".to_string(),"psm1".to_string(),"psd1".to_string()],
  scope: Scope { a: 844987570847744, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_4".to_string(), ContextId { index: 8580 });
    v.insert("#anon_scriptblock_0".to_string(), ContextId { index: 8583 });
    v.insert("commands".to_string(), ContextId { index: 8590 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8576 });
    v.insert("#anon_interpolatedStringContent_0".to_string(), ContextId { index: 8574 });
    v.insert("function".to_string(), ContextId { index: 8595 });
    v.insert("#anon_doubleQuotedString_0".to_string(), ContextId { index: 8571 });
    v.insert("attribute".to_string(), ContextId { index: 8589 });
    v.insert("doubleQuotedStringEscapes".to_string(), ContextId { index: 8594 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 8579 });
    v.insert("unicodeEscape".to_string(), ContextId { index: 8603 });
    v.insert("numericConstant".to_string(), ContextId { index: 8600 });
    v.insert("#anon_commentLine_0".to_string(), ContextId { index: 8570 });
    v.insert("#anon_hashtable_0".to_string(), ContextId { index: 8573 });
    v.insert("__main".to_string(), ContextId { index: 8587 });
    v.insert("interpolatedStringContent".to_string(), ContextId { index: 8597 });
    v.insert("#anon_attribute_0".to_string(), ContextId { index: 8568 });
    v.insert("UsingDirective".to_string(), ContextId { index: 8586 });
    v.insert("RequiresDirective".to_string(), ContextId { index: 8585 });
    v.insert("doubleQuotedString".to_string(), ContextId { index: 8593 });
    v.insert("__start".to_string(), ContextId { index: 8588 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8577 });
    v.insert("main".to_string(), ContextId { index: 8599 });
    v.insert("type".to_string(), ContextId { index: 8602 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 8581 });
    v.insert("scriptblock".to_string(), ContextId { index: 8601 });
    v.insert("#anon_interpolation_0".to_string(), ContextId { index: 8575 });
    v.insert("variable".to_string(), ContextId { index: 8604 });
    v.insert("#anon_attribute_1".to_string(), ContextId { index: 8569 });
    v.insert("#anon_function_0".to_string(), ContextId { index: 8572 });
    v.insert("#anon_type_0".to_string(), ContextId { index: 8584 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 8582 });
    v.insert("hashtable".to_string(), ContextId { index: 8596 });
    v.insert("interpolation".to_string(), ContextId { index: 8598 });
    v.insert("variableNoProperty".to_string(), ContextId { index: 8605 });
    v.insert("commentEmbeddedDocs".to_string(), ContextId { index: 8591 });
    v.insert("commentLine".to_string(), ContextId { index: 8592 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 8578 });
    v.insert("#anon_RequiresDirective_0".to_string(), ContextId { index: 8567 });
    v
  }
} }