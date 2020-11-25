
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SSH Config".to_string(),
  file_extensions: vec!["ssh_config".to_string()],
  scope: Scope { a: 845116419866624, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_localcommand_1".to_string(), ContextId { index: 10261 });
    v.insert("#anon_proxycommand_0".to_string(), ContextId { index: 10268 });
    v.insert("#anon_proxycommand_1".to_string(), ContextId { index: 10269 });
    v.insert("tokens".to_string(), ContextId { index: 10296 });
    v.insert("boolean-value".to_string(), ContextId { index: 10274 });
    v.insert("main".to_string(), ContextId { index: 10286 });
    v.insert("match-conditions".to_string(), ContextId { index: 10289 });
    v.insert("#anon_match-conditions_3".to_string(), ContextId { index: 10265 });
    v.insert("match".to_string(), ContextId { index: 10287 });
    v.insert("__main".to_string(), ContextId { index: 10270 });
    v.insert("comments".to_string(), ContextId { index: 10276 });
    v.insert("#anon_match-conditions_5".to_string(), ContextId { index: 10267 });
    v.insert("boolean-value-with-typing".to_string(), ContextId { index: 10275 });
    v.insert("#anon_match-conditions_0".to_string(), ContextId { index: 10262 });
    v.insert("generic-option-includes".to_string(), ContextId { index: 10278 });
    v.insert("host-body".to_string(), ContextId { index: 10281 });
    v.insert("#anon_match-conditions_4".to_string(), ContextId { index: 10266 });
    v.insert("host-aliases".to_string(), ContextId { index: 10280 });
    v.insert("hostname".to_string(), ContextId { index: 10283 });
    v.insert("generic-option".to_string(), ContextId { index: 10277 });
    v.insert("boolean-option-plus".to_string(), ContextId { index: 10273 });
    v.insert("match-body".to_string(), ContextId { index: 10288 });
    v.insert("match-exec-tokens".to_string(), ContextId { index: 10290 });
    v.insert("pop-before-next-host".to_string(), ContextId { index: 10293 });
    v.insert("#anon_generic-option_0".to_string(), ContextId { index: 10256 });
    v.insert("#anon_hostname_0".to_string(), ContextId { index: 10259 });
    v.insert("__start".to_string(), ContextId { index: 10271 });
    v.insert("localcommand".to_string(), ContextId { index: 10285 });
    v.insert("hostname-or-ip-value".to_string(), ContextId { index: 10284 });
    v.insert("boolean-option".to_string(), ContextId { index: 10272 });
    v.insert("pop-nl-as-value".to_string(), ContextId { index: 10294 });
    v.insert("proxycommand".to_string(), ContextId { index: 10295 });
    v.insert("host".to_string(), ContextId { index: 10279 });
    v.insert("pop-before-match-option".to_string(), ContextId { index: 10292 });
    v.insert("#anon_localcommand_0".to_string(), ContextId { index: 10260 });
    v.insert("#anon_boolean-option-plus_0".to_string(), ContextId { index: 10254 });
    v.insert("#anon_match-conditions_1".to_string(), ContextId { index: 10263 });
    v.insert("host-patterns".to_string(), ContextId { index: 10282 });
    v.insert("#anon_boolean-option_0".to_string(), ContextId { index: 10255 });
    v.insert("#anon_generic-option_1".to_string(), ContextId { index: 10257 });
    v.insert("#anon_generic-option_2".to_string(), ContextId { index: 10258 });
    v.insert("#anon_match-conditions_2".to_string(), ContextId { index: 10264 });
    v.insert("options".to_string(), ContextId { index: 10291 });
    v
  }
} }