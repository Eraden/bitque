
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "R".to_string(),
  file_extensions: vec!["R".to_string(),"r".to_string(),"Rprofile".to_string()],
  scope: Scope { a: 844699808038912, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("var".to_string(), "(?:[a-zA-Z._][a-zA-Z0-9._]*|`[^`]+`)".to_string());
    v.insert("exponent".to_string(), "(?:[eE][-+]?\\d+)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("codesection".to_string(), ContextId { index: 4665 });
    v.insert("roxygen".to_string(), ContextId { index: 4676 });
    v.insert("#anon_function-call-arguments_0".to_string(), ContextId { index: 4651 });
    v.insert("__main".to_string(), ContextId { index: 4660 });
    v.insert("builtin-functions".to_string(), ContextId { index: 4664 });
    v.insert("__start".to_string(), ContextId { index: 4661 });
    v.insert("#anon_roxygen_0".to_string(), ContextId { index: 4657 });
    v.insert("#anon_brackets_3".to_string(), ContextId { index: 4649 });
    v.insert("brackets".to_string(), ContextId { index: 4663 });
    v.insert("#anon_lambda-functions_1".to_string(), ContextId { index: 4655 });
    v.insert("lambda-functions".to_string(), ContextId { index: 4673 });
    v.insert("#anon_accessor_0".to_string(), ContextId { index: 4645 });
    v.insert("function-call-arguments".to_string(), ContextId { index: 4668 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 4659 });
    v.insert("#anon_function-declarations_0".to_string(), ContextId { index: 4653 });
    v.insert("general-variables".to_string(), ContextId { index: 4671 });
    v.insert("main".to_string(), ContextId { index: 4674 });
    v.insert("keywords".to_string(), ContextId { index: 4672 });
    v.insert("accessor".to_string(), ContextId { index: 4662 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 4658 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 4646 });
    v.insert("constants".to_string(), ContextId { index: 4667 });
    v.insert("strings".to_string(), ContextId { index: 4678 });
    v.insert("comments".to_string(), ContextId { index: 4666 });
    v.insert("storage-types".to_string(), ContextId { index: 4677 });
    v.insert("#anon_brackets_2".to_string(), ContextId { index: 4648 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 4650 });
    v.insert("function-calls".to_string(), ContextId { index: 4669 });
    v.insert("#anon_lambda-functions_0".to_string(), ContextId { index: 4654 });
    v.insert("#anon_function-call-arguments_1".to_string(), ContextId { index: 4652 });
    v.insert("function-declarations".to_string(), ContextId { index: 4670 });
    v.insert("operators".to_string(), ContextId { index: 4675 });
    v.insert("#anon_lambda-functions_2".to_string(), ContextId { index: 4656 });
    v.insert("#anon_brackets_1".to_string(), ContextId { index: 4647 });
    v
  }
} }