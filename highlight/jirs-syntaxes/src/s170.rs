
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SSH Common".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 282157878083584, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("ipv6".to_string(), "(?xi:\n  (?:::(?:ffff(?::0{1,4}){0,1}:){0,1}{{ipv4}})          # ::255.255.255.255  ::ffff:255.255.255.255  ::ffff:0:255.255.255.255 (IPv4-mapped IPv6 addresses and IPv4-translated addresses)\n  |(?:(?:[0-9a-f]{1,4}:){1,4}:{{ipv4}})                 # 2001:db8:3:4::192.0.2.33  64:ff9b::192.0.2.33                       (IPv4-Embedded IPv6 Address)\n  |(?:fe80:(?::[0-9a-f]{1,4}){0,4}%[0-9a-z]{1,})        # fe80::7:8%eth0     fe80::7:8%1                                      (link-local IPv6 addresses with zone index)\n  |(?:(?:[0-9a-f]{1,4}:){7,7}    [0-9a-f]{1,4})         # 1:2:3:4:5:6:7:8\n  |   (?:[0-9a-f]{1,4}:      (?::[0-9a-f]{1,4}){1,6})   # 1::3:4:5:6:7:8     1::3:4:5:6:7:8   1::8\n  |(?:(?:[0-9a-f]{1,4}:){1,2}(?::[0-9a-f]{1,4}){1,5})   # 1::4:5:6:7:8       1:2::4:5:6:7:8   1:2::8\n  |(?:(?:[0-9a-f]{1,4}:){1,3}(?::[0-9a-f]{1,4}){1,4})   # 1::5:6:7:8         1:2:3::5:6:7:8   1:2:3::8\n  |(?:(?:[0-9a-f]{1,4}:){1,4}(?::[0-9a-f]{1,4}){1,3})   # 1::6:7:8           1:2:3:4::6:7:8   1:2:3:4::8\n  |(?:(?:[0-9a-f]{1,4}:){1,5}(?::[0-9a-f]{1,4}){1,2})   # 1::7:8             1:2:3:4:5::7:8   1:2:3:4:5::8\n  |(?:(?:[0-9a-f]{1,4}:){1,6}   :[0-9a-f]{1,4})         # 1::8               1:2:3:4:5:6::8   1:2:3:4:5:6::8\n  |(?:(?:[0-9a-f]{1,4}:){1,7}   :)                      # 1::                                 1:2:3:4:5:6:7::\n  |(?::(?:(?::[0-9a-f]{1,4}){1,7}|:))                   # ::2:3:4:5:6:7:8    ::2:3:4:5:6:7:8  ::8       ::\n)".to_string());
    v.insert("zero_to_255".to_string(), "(?:(?:25[0-5])|(?:2[0-4][0-9])|(?:1[0-9][0-9])|(?:[1-9][0-9])|[0-9])".to_string());
    v.insert("ipv4".to_string(), "(?:(?:{{zero_to_255}}\\.){3}{{zero_to_255}})".to_string());
    v.insert("ssh_fingerprint".to_string(), "(?:AAAA(?:E2V|[BC]3N)[\\w+/]+={0,3})".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("bytes-values".to_string(), ContextId { index: 10237 });
    v.insert("mac-addresses".to_string(), ContextId { index: 10247 });
    v.insert("ipv6-square-bracket".to_string(), ContextId { index: 10245 });
    v.insert("__start".to_string(), ContextId { index: 10236 });
    v.insert("#anon_comments-semicolon_0".to_string(), ContextId { index: 10233 });
    v.insert("comments-number-sign".to_string(), ContextId { index: 10238 });
    v.insert("ip-addresses-with-cidr".to_string(), ContextId { index: 10241 });
    v.insert("comments-semicolon".to_string(), ContextId { index: 10239 });
    v.insert("ipv6-with-cidr".to_string(), ContextId { index: 10246 });
    v.insert("pop-before-nl".to_string(), ContextId { index: 10249 });
    v.insert("pop-nl".to_string(), ContextId { index: 10250 });
    v.insert("ssh-fingerprint-with-label".to_string(), ContextId { index: 10252 });
    v.insert("__main".to_string(), ContextId { index: 10235 });
    v.insert("ip-addresses".to_string(), ContextId { index: 10240 });
    v.insert("ipv4-with-cidr".to_string(), ContextId { index: 10243 });
    v.insert("#anon_ssh-fingerprint-with-label_0".to_string(), ContextId { index: 10234 });
    v.insert("ipv4".to_string(), ContextId { index: 10242 });
    v.insert("ipv6".to_string(), ContextId { index: 10244 });
    v.insert("#anon_comments-number-sign_0".to_string(), ContextId { index: 10232 });
    v.insert("ssh-fingerprint".to_string(), ContextId { index: 10251 });
    v.insert("main".to_string(), ContextId { index: 10248 });
    v.insert("time-values".to_string(), ContextId { index: 10253 });
    v
  }
} }