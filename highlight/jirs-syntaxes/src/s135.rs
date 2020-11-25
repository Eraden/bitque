
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "nginx".to_string(),
  file_extensions: vec!["conf.erb".to_string(),"conf".to_string(),"nginx.conf".to_string(),"mime.types".to_string(),"fastcgi_params".to_string(),"scgi_params".to_string(),"uwsgi_params".to_string()],
  scope: Scope { a: 844966096011264, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("variables".to_string(), ContextId { index: 8394 });
    v.insert("#anon_main_37".to_string(), ContextId { index: 8372 });
    v.insert("#anon_main_13".to_string(), ContextId { index: 8346 });
    v.insert("#anon_main_18".to_string(), ContextId { index: 8351 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 8353 });
    v.insert("#anon_main_31".to_string(), ContextId { index: 8366 });
    v.insert("main".to_string(), ContextId { index: 8392 });
    v.insert("#anon_main_38".to_string(), ContextId { index: 8373 });
    v.insert("#anon_main_34".to_string(), ContextId { index: 8369 });
    v.insert("#anon_main_12".to_string(), ContextId { index: 8345 });
    v.insert("__start".to_string(), ContextId { index: 8391 });
    v.insert("#anon_main_43".to_string(), ContextId { index: 8379 });
    v.insert("#anon_main_24".to_string(), ContextId { index: 8358 });
    v.insert("#anon_main_44".to_string(), ContextId { index: 8380 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 8383 });
    v.insert("values".to_string(), ContextId { index: 8393 });
    v.insert("#anon_main_39".to_string(), ContextId { index: 8374 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 8375 });
    v.insert("#anon_main_15".to_string(), ContextId { index: 8348 });
    v.insert("#anon_main_14".to_string(), ContextId { index: 8347 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 8386 });
    v.insert("#anon_main_35".to_string(), ContextId { index: 8370 });
    v.insert("__main".to_string(), ContextId { index: 8390 });
    v.insert("#anon_main_19".to_string(), ContextId { index: 8352 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 8364 });
    v.insert("#anon_main_20".to_string(), ContextId { index: 8354 });
    v.insert("#anon_main_45".to_string(), ContextId { index: 8381 });
    v.insert("#anon_main_25".to_string(), ContextId { index: 8359 });
    v.insert("#anon_main_11".to_string(), ContextId { index: 8344 });
    v.insert("#anon_main_32".to_string(), ContextId { index: 8367 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8342 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8341 });
    v.insert("#anon_main_10".to_string(), ContextId { index: 8343 });
    v.insert("#anon_main_28".to_string(), ContextId { index: 8362 });
    v.insert("#anon_main_27".to_string(), ContextId { index: 8361 });
    v.insert("#anon_main_17".to_string(), ContextId { index: 8350 });
    v.insert("#anon_main_33".to_string(), ContextId { index: 8368 });
    v.insert("#anon_main_42".to_string(), ContextId { index: 8378 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 8384 });
    v.insert("#anon_main_46".to_string(), ContextId { index: 8382 });
    v.insert("#anon_main_29".to_string(), ContextId { index: 8363 });
    v.insert("#anon_main_23".to_string(), ContextId { index: 8357 });
    v.insert("#anon_main_40".to_string(), ContextId { index: 8376 });
    v.insert("#anon_main_30".to_string(), ContextId { index: 8365 });
    v.insert("#anon_main_9".to_string(), ContextId { index: 8387 });
    v.insert("#anon_main_21".to_string(), ContextId { index: 8355 });
    v.insert("#anon_main_22".to_string(), ContextId { index: 8356 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 8385 });
    v.insert("#anon_main_16".to_string(), ContextId { index: 8349 });
    v.insert("#anon_main_26".to_string(), ContextId { index: 8360 });
    v.insert("#anon_main_41".to_string(), ContextId { index: 8377 });
    v.insert("#anon_values_0".to_string(), ContextId { index: 8388 });
    v.insert("#anon_values_1".to_string(), ContextId { index: 8389 });
    v.insert("#anon_main_36".to_string(), ContextId { index: 8371 });
    v
  }
} }