
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Protocol Buffer".to_string(),
  file_extensions: vec!["proto".to_string(),"protodevel".to_string()],
  scope: Scope { a: 844991865815040, b: 0 },
  first_line_match: Some("^(syntax)\\s*(=)\\s*(\"proto\\d\")\\s*(;)\\s*$".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("ident".to_string(), "\\b([A-Za-z][A-Za-z0-9_]*)\\b".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("oneof_body".to_string(), ContextId { index: 8650 });
    v.insert("oneof".to_string(), ContextId { index: 8649 });
    v.insert("comments".to_string(), ContextId { index: 8630 });
    v.insert("enum_body".to_string(), ContextId { index: 8632 });
    v.insert("enum".to_string(), ContextId { index: 8631 });
    v.insert("#anon_full_option_name_0".to_string(), ContextId { index: 8611 });
    v.insert("rpc_returns".to_string(), ContextId { index: 8661 });
    v.insert("#anon_enum_0".to_string(), ContextId { index: 8608 });
    v.insert("#anon_message_0".to_string(), ContextId { index: 8615 });
    v.insert("message".to_string(), ContextId { index: 8643 });
    v.insert("field".to_string(), ContextId { index: 8635 });
    v.insert("#anon_import_0".to_string(), ContextId { index: 8612 });
    v.insert("#anon_service_0".to_string(), ContextId { index: 8623 });
    v.insert("#anon_extend_body_or_pop_0".to_string(), ContextId { index: 8609 });
    v.insert("#anon_service_1".to_string(), ContextId { index: 8624 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 8606 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8614 });
    v.insert("prototype".to_string(), ContextId { index: 8656 });
    v.insert("#anon_option_value_0".to_string(), ContextId { index: 8618 });
    v.insert("#anon_field_attributes_or_pop_0".to_string(), ContextId { index: 8610 });
    v.insert("message_body".to_string(), ContextId { index: 8644 });
    v.insert("or_pop".to_string(), ContextId { index: 8654 });
    v.insert("field_end_or_pop".to_string(), ContextId { index: 8637 });
    v.insert("full_option_name".to_string(), ContextId { index: 8639 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8613 });
    v.insert("#anon_package_0".to_string(), ContextId { index: 8619 });
    v.insert("main".to_string(), ContextId { index: 8641 });
    v.insert("extend".to_string(), ContextId { index: 8633 });
    v.insert("map_type".to_string(), ContextId { index: 8642 });
    v.insert("number_or_pop".to_string(), ContextId { index: 8646 });
    v.insert("assignment_or_pop".to_string(), ContextId { index: 8629 });
    v.insert("rpc_body".to_string(), ContextId { index: 8659 });
    v.insert("rpc".to_string(), ContextId { index: 8658 });
    v.insert("field_attributes_or_pop".to_string(), ContextId { index: 8636 });
    v.insert("string".to_string(), ContextId { index: 8664 });
    v.insert("#anon_option_0".to_string(), ContextId { index: 8617 });
    v.insert("#anon_rpc_param_0".to_string(), ContextId { index: 8622 });
    v.insert("reserved".to_string(), ContextId { index: 8657 });
    v.insert("package".to_string(), ContextId { index: 8655 });
    v.insert("option".to_string(), ContextId { index: 8651 });
    v.insert("import".to_string(), ContextId { index: 8640 });
    v.insert("#anon_oneof_0".to_string(), ContextId { index: 8616 });
    v.insert("rpc_param".to_string(), ContextId { index: 8660 });
    v.insert("option_value".to_string(), ContextId { index: 8653 });
    v.insert("message_type_or_pop".to_string(), ContextId { index: 8645 });
    v.insert("numeric_range".to_string(), ContextId { index: 8647 });
    v.insert("#anon_rpc_body_0".to_string(), ContextId { index: 8621 });
    v.insert("#anon_syntax_0".to_string(), ContextId { index: 8626 });
    v.insert("semicolon_or_pop".to_string(), ContextId { index: 8662 });
    v.insert("service".to_string(), ContextId { index: 8663 });
    v.insert("__main".to_string(), ContextId { index: 8627 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 8607 });
    v.insert("__start".to_string(), ContextId { index: 8628 });
    v.insert("field_name_or_pop".to_string(), ContextId { index: 8638 });
    v.insert("#anon_reserved_0".to_string(), ContextId { index: 8620 });
    v.insert("one_liner".to_string(), ContextId { index: 8648 });
    v.insert("option_inline".to_string(), ContextId { index: 8652 });
    v.insert("syntax".to_string(), ContextId { index: 8665 });
    v.insert("extend_body_or_pop".to_string(), ContextId { index: 8634 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 8625 });
    v
  }
} }