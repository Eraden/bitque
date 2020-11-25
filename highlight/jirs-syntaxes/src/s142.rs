
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Protocol Buffer (TEXT)".to_string(),
  file_extensions: vec!["pb.txt".to_string(),"proto.text".to_string(),"textpb".to_string(),"pbtxt".to_string(),"prototxt".to_string()],
  scope: Scope { a: 282046207361024, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("exp".to_string(), "(?i:e(?:\\+|-)?{{integer}})".to_string());
    v.insert("integer".to_string(), "(?:\\+|-)?(?:0|[1-9]\\d*)".to_string());
    v.insert("field_name".to_string(), "\\b([A-Za-z][A-Za-z0-9_]*)\\b".to_string());
    v.insert("stringEscape".to_string(), "(?:\\\\(?:[\'\"\\\\/abfnrtv?]|[0-9]{3}|(?i:u|x)[0-9A-Fa-f]+))".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_field_value_or_pop_0".to_string(), ContextId { index: 8669 });
    v.insert("comma_or_pop".to_string(), ContextId { index: 8678 });
    v.insert("field".to_string(), ContextId { index: 8682 });
    v.insert("#anon_field_value_or_pop_1".to_string(), ContextId { index: 8670 });
    v.insert("#anon_string_single_multiline_0".to_string(), ContextId { index: 8673 });
    v.insert("field_value_or_pop".to_string(), ContextId { index: 8684 });
    v.insert("enum_value".to_string(), ContextId { index: 8681 });
    v.insert("main".to_string(), ContextId { index: 8685 });
    v.insert("number".to_string(), ContextId { index: 8686 });
    v.insert("or_pop".to_string(), ContextId { index: 8687 });
    v.insert("#anon_field_0".to_string(), ContextId { index: 8668 });
    v.insert("#anon_array_0".to_string(), ContextId { index: 8666 });
    v.insert("field_sep_or_pop".to_string(), ContextId { index: 8683 });
    v.insert("#anon_string_single_multiline_1".to_string(), ContextId { index: 8674 });
    v.insert("prototype".to_string(), ContextId { index: 8688 });
    v.insert("string_double_multiline".to_string(), ContextId { index: 8689 });
    v.insert("__main".to_string(), ContextId { index: 8675 });
    v.insert("#anon_string_double_multiline_1".to_string(), ContextId { index: 8672 });
    v.insert("array".to_string(), ContextId { index: 8677 });
    v.insert("comments".to_string(), ContextId { index: 8679 });
    v.insert("constant".to_string(), ContextId { index: 8680 });
    v.insert("string_single_multiline".to_string(), ContextId { index: 8690 });
    v.insert("#anon_string_double_multiline_0".to_string(), ContextId { index: 8671 });
    v.insert("__start".to_string(), ContextId { index: 8676 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 8667 });
    v
  }
} }