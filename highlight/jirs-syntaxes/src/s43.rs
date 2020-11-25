
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Lisp".to_string(),
  file_extensions: vec!["lisp".to_string(),"cl".to_string(),"clisp".to_string(),"l".to_string(),"mud".to_string(),"el".to_string(),"scm".to_string(),"ss".to_string(),"lsp".to_string(),"fasl".to_string()],
  scope: Scope { a: 844622498627584, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("comments".to_string(), ContextId { index: 3026 });
    v.insert("main".to_string(), ContextId { index: 3031 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 3021 });
    v.insert("control".to_string(), ContextId { index: 3028 });
    v.insert("numbers".to_string(), ContextId { index: 3032 });
    v.insert("functions".to_string(), ContextId { index: 3030 });
    v.insert("parens".to_string(), ContextId { index: 3034 });
    v.insert("operators".to_string(), ContextId { index: 3033 });
    v.insert("#anon_block-comment_0".to_string(), ContextId { index: 3020 });
    v.insert("__main".to_string(), ContextId { index: 3023 });
    v.insert("expressions".to_string(), ContextId { index: 3029 });
    v.insert("block-comment".to_string(), ContextId { index: 3025 });
    v.insert("strings".to_string(), ContextId { index: 3035 });
    v.insert("variables".to_string(), ContextId { index: 3036 });
    v.insert("constants".to_string(), ContextId { index: 3027 });
    v.insert("__start".to_string(), ContextId { index: 3024 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 3022 });
    v
  }
} }