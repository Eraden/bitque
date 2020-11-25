
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Stylus".to_string(),
  file_extensions: vec!["styl".to_string(),"stylus".to_string()],
  scope: Scope { a: 845047700389888, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_function-call_0".to_string(), ContextId { index: 9177 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 9188 });
    v.insert("function-call".to_string(), ContextId { index: 9208 });
    v.insert("language-property-value-constants".to_string(), ContextId { index: 9216 });
    v.insert("string-quoted".to_string(), ContextId { index: 9229 });
    v.insert("#anon_return_0".to_string(), ContextId { index: 9197 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 9191 });
    v.insert("conditionals".to_string(), ContextId { index: 9205 });
    v.insert("language-keywords".to_string(), ContextId { index: 9214 });
    v.insert("#anon_pseudo-not_0".to_string(), ContextId { index: 9195 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 9190 });
    v.insert("pseudo-nth".to_string(), ContextId { index: 9225 });
    v.insert("variable".to_string(), ContextId { index: 9231 });
    v.insert("#anon_url_0".to_string(), ContextId { index: 9198 });
    v.insert("pseudo-not".to_string(), ContextId { index: 9224 });
    v.insert("#anon_pseudo-nth_0".to_string(), ContextId { index: 9196 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 9175 });
    v.insert("#anon_attribute-selector_0".to_string(), ContextId { index: 9172 });
    v.insert("#anon_media-query_0".to_string(), ContextId { index: 9193 });
    v.insert("iteration".to_string(), ContextId { index: 9212 });
    v.insert("#anon_interpolation_0".to_string(), ContextId { index: 9181 });
    v.insert("language-constants".to_string(), ContextId { index: 9213 });
    v.insert("#anon_media-query-list_0".to_string(), ContextId { index: 9192 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 9183 });
    v.insert("__main".to_string(), ContextId { index: 9199 });
    v.insert("single-line-comment".to_string(), ContextId { index: 9228 });
    v.insert("url".to_string(), ContextId { index: 9230 });
    v.insert("media-query-properties".to_string(), ContextId { index: 9220 });
    v.insert("#anon_hash-definition_0".to_string(), ContextId { index: 9179 });
    v.insert("#anon_attribute-selector_1".to_string(), ContextId { index: 9173 });
    v.insert("__start".to_string(), ContextId { index: 9200 });
    v.insert("pseudo".to_string(), ContextId { index: 9223 });
    v.insert("expression".to_string(), ContextId { index: 9207 });
    v.insert("return".to_string(), ContextId { index: 9226 });
    v.insert("media-query-list".to_string(), ContextId { index: 9219 });
    v.insert("#anon_media-query_1".to_string(), ContextId { index: 9194 });
    v.insert("#anon_hash-access_0".to_string(), ContextId { index: 9178 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 9184 });
    v.insert("hash-definition".to_string(), ContextId { index: 9210 });
    v.insert("escape".to_string(), ContextId { index: 9206 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 9186 });
    v.insert("#anon_hash-definition_1".to_string(), ContextId { index: 9180 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 9187 });
    v.insert("numeric-values".to_string(), ContextId { index: 9221 });
    v.insert("#anon_conditionals_0".to_string(), ContextId { index: 9176 });
    v.insert("language-operators".to_string(), ContextId { index: 9215 });
    v.insert("interpolation".to_string(), ContextId { index: 9211 });
    v.insert("main".to_string(), ContextId { index: 9217 });
    v.insert("comma".to_string(), ContextId { index: 9203 });
    v.insert("selector-components".to_string(), ContextId { index: 9227 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 9185 });
    v.insert("attribute-selector".to_string(), ContextId { index: 9201 });
    v.insert("#anon_color-values_0".to_string(), ContextId { index: 9174 });
    v.insert("#anon_iteration_0".to_string(), ContextId { index: 9182 });
    v.insert("color-values".to_string(), ContextId { index: 9202 });
    v.insert("property-reference".to_string(), ContextId { index: 9222 });
    v.insert("media-query".to_string(), ContextId { index: 9218 });
    v.insert("comments".to_string(), ContextId { index: 9204 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 9189 });
    v.insert("hash-access".to_string(), ContextId { index: 9209 });
    v
  }
} }