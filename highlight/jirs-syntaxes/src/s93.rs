
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "CMake".to_string(),
  file_extensions: vec!["CMakeLists.txt".to_string(),"cmake".to_string()],
  scope: Scope { a: 844802887254016, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), "\\b[[:alpha:]_][[:alnum:]_]*\\b".to_string());
    v.insert("unquoted_argument".to_string(), "{{unquoted_argument_element}}+".to_string());
    v.insert("generic_named_parameter".to_string(), "\\b@?[A-Z_][A-Z0-9_]*(?=[^\\w\\-<>=\\$])".to_string());
    v.insert("unquoted_argument_element".to_string(), "[^ \t()#\"\\\']".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_foreach_1".to_string(), ContextId { index: 6171 });
    v.insert("#anon_endif_1".to_string(), ContextId { index: 6165 });
    v.insert("break-args".to_string(), ContextId { index: 6203 });
    v.insert("endfunction".to_string(), ContextId { index: 6213 });
    v.insert("#anon_generator-expression_1".to_string(), ContextId { index: 6175 });
    v.insert("foreach-body".to_string(), ContextId { index: 6220 });
    v.insert("macro".to_string(), ContextId { index: 6236 });
    v.insert("set-args-rest".to_string(), ContextId { index: 6244 });
    v.insert("#anon_break_1".to_string(), ContextId { index: 6151 });
    v.insert("#anon_generic-command_1".to_string(), ContextId { index: 6177 });
    v.insert("#anon_endwhile_0".to_string(), ContextId { index: 6168 });
    v.insert("foreach-args".to_string(), ContextId { index: 6219 });
    v.insert("continue-args".to_string(), ContextId { index: 6207 });
    v.insert("#anon_set-args-rest_0".to_string(), ContextId { index: 6187 });
    v.insert("string".to_string(), ContextId { index: 6245 });
    v.insert("#anon_if-args_1".to_string(), ContextId { index: 6179 });
    v.insert("#anon_elseif_0".to_string(), ContextId { index: 6158 });
    v.insert("generator-expression".to_string(), ContextId { index: 6225 });
    v.insert("function-args-first".to_string(), ContextId { index: 6222 });
    v.insert("#anon_string-raw_0".to_string(), ContextId { index: 6192 });
    v.insert("#anon_variable-substitution_0".to_string(), ContextId { index: 6195 });
    v.insert("#anon_break_0".to_string(), ContextId { index: 6150 });
    v.insert("#anon_endfunction_1".to_string(), ContextId { index: 6163 });
    v.insert("args-illegal-boilerplate".to_string(), ContextId { index: 6201 });
    v.insert("#anon_elseif_1".to_string(), ContextId { index: 6159 });
    v.insert("include-args".to_string(), ContextId { index: 6235 });
    v.insert("string-raw".to_string(), ContextId { index: 6248 });
    v.insert("function-args-rest".to_string(), ContextId { index: 6223 });
    v.insert("variable-substitution".to_string(), ContextId { index: 6251 });
    v.insert("__main".to_string(), ContextId { index: 6198 });
    v.insert("string-unquoted".to_string(), ContextId { index: 6249 });
    v.insert("#anon_macro_0".to_string(), ContextId { index: 6185 });
    v.insert("#anon_generator-expression_0".to_string(), ContextId { index: 6174 });
    v.insert("args-common".to_string(), ContextId { index: 6200 });
    v.insert("comment-line".to_string(), ContextId { index: 6205 });
    v.insert("continue".to_string(), ContextId { index: 6206 });
    v.insert("generic-command".to_string(), ContextId { index: 6228 });
    v.insert("#anon_if_0".to_string(), ContextId { index: 6180 });
    v.insert("macro-args-first".to_string(), ContextId { index: 6237 });
    v.insert("#anon_include-args_0".to_string(), ContextId { index: 6182 });
    v.insert("set-args-first".to_string(), ContextId { index: 6243 });
    v.insert("generator-expression-common".to_string(), ContextId { index: 6226 });
    v.insert("#anon_string-quoted-single_0".to_string(), ContextId { index: 6191 });
    v.insert("#anon_comment-line_0".to_string(), ContextId { index: 6153 });
    v.insert("break".to_string(), ContextId { index: 6202 });
    v.insert("generic-args".to_string(), ContextId { index: 6227 });
    v.insert("if-args".to_string(), ContextId { index: 6231 });
    v.insert("#anon_function_0".to_string(), ContextId { index: 6172 });
    v.insert("unquoted-argument-or-keyword".to_string(), ContextId { index: 6250 });
    v.insert("escape-sequences".to_string(), ContextId { index: 6217 });
    v.insert("while".to_string(), ContextId { index: 6252 });
    v.insert("#anon_endmacro_0".to_string(), ContextId { index: 6166 });
    v.insert("#anon_if-args_0".to_string(), ContextId { index: 6178 });
    v.insert("#anon_include_1".to_string(), ContextId { index: 6184 });
    v.insert("#anon_string-quoted-double_0".to_string(), ContextId { index: 6190 });
    v.insert("elseif".to_string(), ContextId { index: 6210 });
    v.insert("#anon_function_1".to_string(), ContextId { index: 6173 });
    v.insert("macro-body".to_string(), ContextId { index: 6239 });
    v.insert("string-quoted-single".to_string(), ContextId { index: 6247 });
    v.insert("endif".to_string(), ContextId { index: 6214 });
    v.insert("foreach".to_string(), ContextId { index: 6218 });
    v.insert("#anon_macro_1".to_string(), ContextId { index: 6186 });
    v.insert("#anon_set_0".to_string(), ContextId { index: 6188 });
    v.insert("#anon_while_0".to_string(), ContextId { index: 6196 });
    v.insert("endwhile".to_string(), ContextId { index: 6216 });
    v.insert("while-body".to_string(), ContextId { index: 6253 });
    v.insert("macro-args-rest".to_string(), ContextId { index: 6238 });
    v.insert("prototype".to_string(), ContextId { index: 6241 });
    v.insert("#anon_generic-command_0".to_string(), ContextId { index: 6176 });
    v.insert("#anon_endwhile_1".to_string(), ContextId { index: 6169 });
    v.insert("#anon_endfunction_0".to_string(), ContextId { index: 6162 });
    v.insert("#anon_set_1".to_string(), ContextId { index: 6189 });
    v.insert("illegal-command".to_string(), ContextId { index: 6233 });
    v.insert("comment-block".to_string(), ContextId { index: 6204 });
    v.insert("__start".to_string(), ContextId { index: 6199 });
    v.insert("else".to_string(), ContextId { index: 6208 });
    v.insert("#anon_continue_1".to_string(), ContextId { index: 6155 });
    v.insert("else-body".to_string(), ContextId { index: 6209 });
    v.insert("if".to_string(), ContextId { index: 6230 });
    v.insert("highlight-semicolon".to_string(), ContextId { index: 6229 });
    v.insert("endforeach".to_string(), ContextId { index: 6212 });
    v.insert("if-body".to_string(), ContextId { index: 6232 });
    v.insert("#anon_include_0".to_string(), ContextId { index: 6183 });
    v.insert("#anon_endforeach_0".to_string(), ContextId { index: 6160 });
    v.insert("#anon_comment-block_0".to_string(), ContextId { index: 6152 });
    v.insert("#anon_else_0".to_string(), ContextId { index: 6156 });
    v.insert("#anon_if_1".to_string(), ContextId { index: 6181 });
    v.insert("#anon_endmacro_1".to_string(), ContextId { index: 6167 });
    v.insert("#anon_foreach_0".to_string(), ContextId { index: 6170 });
    v.insert("#anon_while_1".to_string(), ContextId { index: 6197 });
    v.insert("elseif-body".to_string(), ContextId { index: 6211 });
    v.insert("function".to_string(), ContextId { index: 6221 });
    v.insert("#anon_continue_0".to_string(), ContextId { index: 6154 });
    v.insert("#anon_string-unquoted_1".to_string(), ContextId { index: 6194 });
    v.insert("#anon_endforeach_1".to_string(), ContextId { index: 6161 });
    v.insert("#anon_endif_0".to_string(), ContextId { index: 6164 });
    v.insert("#anon_string-unquoted_0".to_string(), ContextId { index: 6193 });
    v.insert("endmacro".to_string(), ContextId { index: 6215 });
    v.insert("function-body".to_string(), ContextId { index: 6224 });
    v.insert("include".to_string(), ContextId { index: 6234 });
    v.insert("main".to_string(), ContextId { index: 6240 });
    v.insert("set".to_string(), ContextId { index: 6242 });
    v.insert("#anon_else_1".to_string(), ContextId { index: 6157 });
    v.insert("string-quoted-double".to_string(), ContextId { index: 6246 });
    v
  }
} }