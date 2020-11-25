
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Swift".to_string(),
  file_extensions: vec!["swift".to_string()],
  scope: Scope { a: 845051995357184, b: 0 },
  first_line_match: Some("^#!/.*\\bswift".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), ContextId { index: 9271 });
    v.insert("ternary-operator".to_string(), ContextId { index: 9298 });
    v.insert("#anon_function-declaration_0".to_string(), ContextId { index: 9238 });
    v.insert("#anon_documentation-comment_0".to_string(), ContextId { index: 9237 });
    v.insert("#anon_code-block_0".to_string(), ContextId { index: 9235 });
    v.insert("boolean".to_string(), ContextId { index: 9253 });
    v.insert("comment".to_string(), ContextId { index: 9258 });
    v.insert("primitive-type".to_string(), ContextId { index: 9289 });
    v.insert("keyword".to_string(), ContextId { index: 9278 });
    v.insert("#anon_generic-parameter-clause_0".to_string(), ContextId { index: 9240 });
    v.insert("branch-statement-keyword".to_string(), ContextId { index: 9254 });
    v.insert("__main".to_string(), ContextId { index: 9244 });
    v.insert("comparative-operator".to_string(), ContextId { index: 9259 });
    v.insert("dictionary-type".to_string(), ContextId { index: 9264 });
    v.insert("block-comment".to_string(), ContextId { index: 9252 });
    v.insert("logical-operator".to_string(), ContextId { index: 9280 });
    v.insert("type-casting-operator".to_string(), ContextId { index: 9300 });
    v.insert("range-operator".to_string(), ContextId { index: 9291 });
    v.insert("overflow-operator".to_string(), ContextId { index: 9287 });
    v.insert("loop-statement-keyword".to_string(), ContextId { index: 9281 });
    v.insert("custom-operator".to_string(), ContextId { index: 9261 });
    v.insert("array-type".to_string(), ContextId { index: 9248 });
    v.insert("floating-point-literal".to_string(), ContextId { index: 9266 });
    v.insert("main".to_string(), ContextId { index: 9282 });
    v.insert("operator-declaration-modifier".to_string(), ContextId { index: 9285 });
    v.insert("remainder-operator".to_string(), ContextId { index: 9292 });
    v.insert("#anon_string-literal_0".to_string(), ContextId { index: 9242 });
    v.insert("control-transfer-statement-keyword".to_string(), ContextId { index: 9260 });
    v.insert("access-level-modifier".to_string(), ContextId { index: 9246 });
    v.insert("nil-literal".to_string(), ContextId { index: 9283 });
    v.insert("__start".to_string(), ContextId { index: 9245 });
    v.insert("collection-type".to_string(), ContextId { index: 9257 });
    v.insert("declaration-modifier".to_string(), ContextId { index: 9263 });
    v.insert("in-line-comment".to_string(), ContextId { index: 9274 });
    v.insert("arithmetic-operator".to_string(), ContextId { index: 9247 });
    v.insert("#anon_attribute_0".to_string(), ContextId { index: 9233 });
    v.insert("documentation-comment".to_string(), ContextId { index: 9265 });
    v.insert("#anon_string-literal_1".to_string(), ContextId { index: 9243 });
    v.insert("import-declaration".to_string(), ContextId { index: 9273 });
    v.insert("integer-literal".to_string(), ContextId { index: 9276 });
    v.insert("operator".to_string(), ContextId { index: 9284 });
    v.insert("special-literal".to_string(), ContextId { index: 9294 });
    v.insert("switch-statement-keyword".to_string(), ContextId { index: 9297 });
    v.insert("type".to_string(), ContextId { index: 9299 });
    v.insert("increment-decrement-operator".to_string(), ContextId { index: 9275 });
    v.insert("bitwise-operator".to_string(), ContextId { index: 9251 });
    v.insert("catch-statement-keyword".to_string(), ContextId { index: 9255 });
    v.insert("if-statement-keyword".to_string(), ContextId { index: 9272 });
    v.insert("optional-type".to_string(), ContextId { index: 9286 });
    v.insert("#anon_parameter-clause_0".to_string(), ContextId { index: 9241 });
    v.insert("parameter-clause".to_string(), ContextId { index: 9288 });
    v.insert("function-result".to_string(), ContextId { index: 9269 });
    v.insert("declaration".to_string(), ContextId { index: 9262 });
    v.insert("code-block".to_string(), ContextId { index: 9256 });
    v.insert("function-body".to_string(), ContextId { index: 9267 });
    v.insert("storage-type".to_string(), ContextId { index: 9295 });
    v.insert("string-literal".to_string(), ContextId { index: 9296 });
    v.insert("shebang-line".to_string(), ContextId { index: 9293 });
    v.insert("#anon_array-type_0".to_string(), ContextId { index: 9232 });
    v.insert("protocol-composition-type".to_string(), ContextId { index: 9290 });
    v.insert("integer-type".to_string(), ContextId { index: 9277 });
    v.insert("attribute".to_string(), ContextId { index: 9250 });
    v.insert("literal".to_string(), ContextId { index: 9279 });
    v.insert("#anon_block-comment_0".to_string(), ContextId { index: 9234 });
    v.insert("assignment-operator".to_string(), ContextId { index: 9249 });
    v.insert("#anon_dictionary-type_0".to_string(), ContextId { index: 9236 });
    v.insert("#anon_function-result_0".to_string(), ContextId { index: 9239 });
    v.insert("function-declaration".to_string(), ContextId { index: 9268 });
    v.insert("generic-parameter-clause".to_string(), ContextId { index: 9270 });
    v
  }
} }