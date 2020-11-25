
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "TOML".to_string(),
  file_extensions: vec!["toml".to_string(),"tml".to_string(),"Cargo.lock".to_string(),"Gopkg.lock".to_string(),"Pipfile".to_string()],
  scope: Scope { a: 845060585291776, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("integer".to_string(), "([\\+\\-]?) (?: [0-9] | [1-9] (?: [0-9] | _ [0-9] )+ )".to_string());
    v.insert("local_date".to_string(), "{{full_date}}".to_string());
    v.insert("time_numoffset".to_string(), "[+-] {{time_hour}} : {{time_minute}}".to_string());
    v.insert("wsnl".to_string(), "([ \\t\\n])*".to_string());
    v.insert("oct_int".to_string(), "0o[0-7](?:[0-7]|_[0-7])*".to_string());
    v.insert("date_month".to_string(), "[0-1][0-9]".to_string());
    v.insert("hex_int".to_string(), "0x{{HEXDIG}}(?:{{HEXDIG}}|_{{HEXDIG}})*".to_string());
    v.insert("local_time".to_string(), "{{partial_time}}".to_string());
    v.insert("time_hour".to_string(), "[0-2][0-9]".to_string());
    v.insert("local_date_time".to_string(), "{{full_date}} [tT ] {{partial_time}}".to_string());
    v.insert("dot_peek_key".to_string(), "{{ws}}(\\.){{ws}}{{peek_key_start}}".to_string());
    v.insert("bin_int".to_string(), "0b[0-1](?:[0-1]|_[0-1])*".to_string());
    v.insert("exp".to_string(), "[eE] [\\+\\-]? {{zero_prefixable_int}}".to_string());
    v.insert("peek_key_start".to_string(), "(?=[A-Za-z0-9_\'\"-])".to_string());
    v.insert("offset_date_time".to_string(), "{{full_date}} [tT ] {{full_time}}".to_string());
    v.insert("time_secfrac".to_string(), "\\.[0-9]+".to_string());
    v.insert("date_time".to_string(), "{{offset_date_time}} | {{local_date_time}} | {{local_date}} | {{local_time}}".to_string());
    v.insert("HEXDIG".to_string(), "[0-9A-Fa-f]".to_string());
    v.insert("time_minute".to_string(), "[0-5][0-9]".to_string());
    v.insert("time_second".to_string(), "[0-6][0-9]".to_string());
    v.insert("frac".to_string(), "\\. {{zero_prefixable_int}}".to_string());
    v.insert("full_time".to_string(), "{{partial_time}} {{time_offset}}".to_string());
    v.insert("date_fullyear".to_string(), "[0-9]{4}".to_string());
    v.insert("full_date".to_string(), "{{date_fullyear}} - {{date_month}} - {{date_mday}}".to_string());
    v.insert("partial_time".to_string(), "{{time_hour}} : {{time_minute}} : {{time_second}} (?: {{time_secfrac}} )?".to_string());
    v.insert("time_offset".to_string(), "(?: [zZ] | {{time_numoffset}} )".to_string());
    v.insert("zero_prefixable_int".to_string(), "[0-9] (?: [0-9] | _ [0-9] )*".to_string());
    v.insert("ws".to_string(), "[ \\t]*".to_string());
    v.insert("date_mday".to_string(), "[0-3][0-9]".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("illegal".to_string(), ContextId { index: 9320 });
    v.insert("key-simple".to_string(), ContextId { index: 9327 });
    v.insert("data-types".to_string(), ContextId { index: 9317 });
    v.insert("__start".to_string(), ContextId { index: 9311 });
    v.insert("integer".to_string(), ContextId { index: 9324 });
    v.insert("comments".to_string(), ContextId { index: 9316 });
    v.insert("maybe-empty-array".to_string(), ContextId { index: 9336 });
    v.insert("__main".to_string(), ContextId { index: 9310 });
    v.insert("inline-keyval-sep".to_string(), ContextId { index: 9321 });
    v.insert("table-array-name".to_string(), ContextId { index: 9339 });
    v.insert("array".to_string(), ContextId { index: 9312 });
    v.insert("#anon_inline-table_0".to_string(), ContextId { index: 9307 });
    v.insert("basic-string".to_string(), ContextId { index: 9314 });
    v.insert("maybe-array-comments".to_string(), ContextId { index: 9332 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 9308 });
    v.insert("maybe-dot-key".to_string(), ContextId { index: 9334 });
    v.insert("table-name-simple".to_string(), ContextId { index: 9342 });
    v.insert("inline-table-keyvals".to_string(), ContextId { index: 9323 });
    v.insert("string-escape".to_string(), ContextId { index: 9338 });
    v.insert("maybe-dot-table-name".to_string(), ContextId { index: 9335 });
    v.insert("#anon_inline-table-keyvals_0".to_string(), ContextId { index: 9306 });
    v.insert("boolean".to_string(), ContextId { index: 9315 });
    v.insert("float".to_string(), ContextId { index: 9319 });
    v.insert("table-name".to_string(), ContextId { index: 9340 });
    v.insert("keyval".to_string(), ContextId { index: 9328 });
    v.insert("#anon_string_1".to_string(), ContextId { index: 9309 });
    v.insert("string".to_string(), ContextId { index: 9337 });
    v.insert("literal-string".to_string(), ContextId { index: 9330 });
    v.insert("tables".to_string(), ContextId { index: 9343 });
    v.insert("key-quoted".to_string(), ContextId { index: 9326 });
    v.insert("#anon_basic-string_0".to_string(), ContextId { index: 9305 });
    v.insert("date-time".to_string(), ContextId { index: 9318 });
    v.insert("key".to_string(), ContextId { index: 9325 });
    v.insert("main".to_string(), ContextId { index: 9331 });
    v.insert("table-name-inc".to_string(), ContextId { index: 9341 });
    v.insert("maybe-comment-eol".to_string(), ContextId { index: 9333 });
    v.insert("keyval-sep".to_string(), ContextId { index: 9329 });
    v.insert("array-sep".to_string(), ContextId { index: 9313 });
    v.insert("#anon_array-sep_0".to_string(), ContextId { index: 9304 });
    v.insert("inline-table".to_string(), ContextId { index: 9322 });
    v
  }
} }