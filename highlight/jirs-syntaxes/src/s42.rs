
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "TeX".to_string(),
  file_extensions: vec!["sty".to_string(),"cls".to_string()],
  scope: Scope { a: 281629595533312, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_block-math_0".to_string(), ContextId { index: 2990 });
    v.insert("math-commands".to_string(), ContextId { index: 3015 });
    v.insert("block-math".to_string(), ContextId { index: 2999 });
    v.insert("comments".to_string(), ContextId { index: 3003 });
    v.insert("#anon_macros_0".to_string(), ContextId { index: 2995 });
    v.insert("macro-braces".to_string(), ContextId { index: 3009 });
    v.insert("__start".to_string(), ContextId { index: 2998 });
    v.insert("#anon_macro-braces_0".to_string(), ContextId { index: 2994 });
    v.insert("catcode".to_string(), ContextId { index: 3002 });
    v.insert("boxes".to_string(), ContextId { index: 3000 });
    v.insert("general-constants".to_string(), ContextId { index: 3006 });
    v.insert("#anon_boxes_0".to_string(), ContextId { index: 2991 });
    v.insert("math-braces".to_string(), ContextId { index: 3012 });
    v.insert("braces".to_string(), ContextId { index: 3001 });
    v.insert("inline-math".to_string(), ContextId { index: 3008 });
    v.insert("prototype".to_string(), ContextId { index: 3019 });
    v.insert("general-commands".to_string(), ContextId { index: 3005 });
    v.insert("math-brackets".to_string(), ContextId { index: 3013 });
    v.insert("main".to_string(), ContextId { index: 3011 });
    v.insert("#anon_braces_0".to_string(), ContextId { index: 2992 });
    v.insert("math-content".to_string(), ContextId { index: 3016 });
    v.insert("math-numerics".to_string(), ContextId { index: 3017 });
    v.insert("__main".to_string(), ContextId { index: 2997 });
    v.insert("macros".to_string(), ContextId { index: 3010 });
    v.insert("#anon_inline-math_0".to_string(), ContextId { index: 2993 });
    v.insert("greeks".to_string(), ContextId { index: 3007 });
    v.insert("math-operators".to_string(), ContextId { index: 3018 });
    v.insert("#anon_math-braces_0".to_string(), ContextId { index: 2996 });
    v.insert("controls".to_string(), ContextId { index: 3004 });
    v.insert("math-characters".to_string(), ContextId { index: 3014 });
    v
  }
} }