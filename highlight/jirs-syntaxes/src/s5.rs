
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Batch File".to_string(),
  file_extensions: vec!["bat".to_string(),"cmd".to_string()],
  scope: Scope { a: 844463584837632, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("set_arithmetic_operators_unquoted".to_string(), "(?:\\+|-|\\*|/|%%|~)".to_string());
    v.insert("set_arithmetic_operators_quoted".to_string(), "(?:\\||<<|>>|&|\\^)".to_string());
    v.insert("colon_comment_start".to_string(), "(?::[+=,;: ])".to_string());
    v.insert("command_terminators".to_string(), "(?=$\\n|[&|><)])".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("comments".to_string(), ContextId { index: 199 });
    v.insert("#anon_inside_command_set_1".to_string(), ContextId { index: 173 });
    v.insert("#anon_inside_command_set_4".to_string(), ContextId { index: 176 });
    v.insert("#anon_inside_command_set_5".to_string(), ContextId { index: 177 });
    v.insert("#anon_inside_command_set_3".to_string(), ContextId { index: 175 });
    v.insert("#anon_variable_0".to_string(), ContextId { index: 183 });
    v.insert("__start".to_string(), ContextId { index: 192 });
    v.insert("numbers".to_string(), ContextId { index: 208 });
    v.insert("#anon_variable_delayed_expansion_0".to_string(), ContextId { index: 187 });
    v.insert("#anon_set_arithmetic_quoted_0".to_string(), ContextId { index: 181 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 182 });
    v.insert("inside_command_set".to_string(), ContextId { index: 203 });
    v.insert("#anon_inside_command_set_2".to_string(), ContextId { index: 174 });
    v.insert("variable_substitution_replacee".to_string(), ContextId { index: 218 });
    v.insert("#anon_variable_delayed_expansion_2".to_string(), ContextId { index: 189 });
    v.insert("set_arithmetic_operator_common".to_string(), ContextId { index: 211 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 169 });
    v.insert("#anon_variable_delayed_expansion_3".to_string(), ContextId { index: 190 });
    v.insert("#anon_command_set_group_already_quoted_0".to_string(), ContextId { index: 166 });
    v.insert("set_arithmetic_operator_quoted".to_string(), ContextId { index: 212 });
    v.insert("__main".to_string(), ContextId { index: 191 });
    v.insert("#anon_inside_command_set_7".to_string(), ContextId { index: 179 });
    v.insert("#anon_commands_0".to_string(), ContextId { index: 168 });
    v.insert("command_set_group_already_quoted".to_string(), ContextId { index: 195 });
    v.insert("#anon_comments_2".to_string(), ContextId { index: 171 });
    v.insert("command_set".to_string(), ContextId { index: 194 });
    v.insert("expressions".to_string(), ContextId { index: 202 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 170 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 180 });
    v.insert("command_set_group_newly_quoted".to_string(), ContextId { index: 196 });
    v.insert("constants".to_string(), ContextId { index: 200 });
    v.insert("inside_variable_substring".to_string(), ContextId { index: 205 });
    v.insert("set_arithmetic_quoted".to_string(), ContextId { index: 213 });
    v.insert("set_arithmetic_unquoted".to_string(), ContextId { index: 214 });
    v.insert("variable_delayed_expansion".to_string(), ContextId { index: 217 });
    v.insert("#anon_variable_2".to_string(), ContextId { index: 185 });
    v.insert("commands".to_string(), ContextId { index: 198 });
    v.insert("#anon_inside_command_set_0".to_string(), ContextId { index: 172 });
    v.insert("main".to_string(), ContextId { index: 207 });
    v.insert("#anon_variable_delayed_expansion_1".to_string(), ContextId { index: 188 });
    v.insert("command_set_group_unquoted".to_string(), ContextId { index: 197 });
    v.insert("#anon_variable_3".to_string(), ContextId { index: 186 });
    v.insert("variable".to_string(), ContextId { index: 216 });
    v.insert("variables".to_string(), ContextId { index: 219 });
    v.insert("#anon_command_set_group_newly_quoted_0".to_string(), ContextId { index: 167 });
    v.insert("#anon_inside_command_set_6".to_string(), ContextId { index: 178 });
    v.insert("comma_separator".to_string(), ContextId { index: 193 });
    v.insert("inside_command_set_arithmetic".to_string(), ContextId { index: 204 });
    v.insert("#anon_variable_1".to_string(), ContextId { index: 184 });
    v.insert("labels".to_string(), ContextId { index: 206 });
    v.insert("operators".to_string(), ContextId { index: 209 });
    v.insert("strings".to_string(), ContextId { index: 215 });
    v.insert("parens".to_string(), ContextId { index: 210 });
    v.insert("escaped_characters".to_string(), ContextId { index: 201 });
    v
  }
} }