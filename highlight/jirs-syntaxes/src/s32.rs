
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "JSON".to_string(),
  file_extensions: vec!["json".to_string(),"sublime-settings".to_string(),"sublime-menu".to_string(),"sublime-keymap".to_string(),"sublime-mousemap".to_string(),"sublime-theme".to_string(),"sublime-build".to_string(),"sublime-project".to_string(),"sublime-completions".to_string(),"sublime-commands".to_string(),"sublime-macro".to_string(),"sublime-color-scheme".to_string(),"ipynb".to_string(),"Pipfile.lock".to_string()],
  scope: Scope { a: 844588138889216, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_object_4".to_string(), ContextId { index: 2193 });
    v.insert("__main".to_string(), ContextId { index: 2194 });
    v.insert("number".to_string(), ContextId { index: 2201 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 2187 });
    v.insert("prototype".to_string(), ContextId { index: 2203 });
    v.insert("#anon_object_2".to_string(), ContextId { index: 2191 });
    v.insert("constant".to_string(), ContextId { index: 2198 });
    v.insert("string".to_string(), ContextId { index: 2204 });
    v.insert("string-escape".to_string(), ContextId { index: 2205 });
    v.insert("__start".to_string(), ContextId { index: 2195 });
    v.insert("main".to_string(), ContextId { index: 2200 });
    v.insert("#anon_array_0".to_string(), ContextId { index: 2186 });
    v.insert("#anon_object_3".to_string(), ContextId { index: 2192 });
    v.insert("#anon_object_0".to_string(), ContextId { index: 2189 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 2188 });
    v.insert("#anon_object_1".to_string(), ContextId { index: 2190 });
    v.insert("comments".to_string(), ContextId { index: 2197 });
    v.insert("object".to_string(), ContextId { index: 2202 });
    v.insert("value".to_string(), ContextId { index: 2206 });
    v.insert("array".to_string(), ContextId { index: 2196 });
    v.insert("inside-string".to_string(), ContextId { index: 2199 });
    v
  }
} }