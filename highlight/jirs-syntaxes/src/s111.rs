
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Email".to_string(),
  file_extensions: vec!["eml".to_string(),"msg".to_string(),"mbx".to_string(),"mboxz".to_string()],
  scope: Scope { a: 281921653309440, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("ipv4".to_string(), "\\b(?:(?:{{zero_to_255}}\\.){3}{{zero_to_255}})\\b".to_string());
    v.insert("boundary_name".to_string(), "(?:[\\w=?:-]*[a-zA-Z0-9][\\w=?:-]*)".to_string());
    v.insert("base64_string".to_string(), "(?x:\n  (?:{{base64_char}}{4})+\n  (?:{{base64_char}}{2}==|\n     {{base64_char}}{3}=)?\n)".to_string());
    v.insert("base64_char".to_string(), "[0-9A-Za-z+/]".to_string());
    v.insert("month3".to_string(), "(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)".to_string());
    v.insert("day3".to_string(), "(?:Mon|Tue|Wed|Thu|Fri|Sat|Sun)".to_string());
    v.insert("ipv6".to_string(), "(?xi:\n  (?:::(?:ffff(?::0{1,4}){0,1}:){0,1}{{ipv4}})          # ::255.255.255.255  ::ffff:255.255.255.255  ::ffff:0:255.255.255.255 (IPv4-mapped IPv6 addresses and IPv4-translated addresses)\n  |(?:(?:[0-9a-f]{1,4}:){1,4}:{{ipv4}})                 # 2001:db8:3:4::192.0.2.33  64:ff9b::192.0.2.33                       (IPv4-Embedded IPv6 Address)\n  |(?:fe80:(?::[0-9a-f]{1,4}){0,4}%[0-9a-z]{1,})        # fe80::7:8%eth0     fe80::7:8%1                                      (link-local IPv6 addresses with zone index)\n  |(?:(?:[0-9a-f]{1,4}:){7,7}    [0-9a-f]{1,4})         # 1:2:3:4:5:6:7:8\n  |   (?:[0-9a-f]{1,4}:      (?::[0-9a-f]{1,4}){1,6})   # 1::3:4:5:6:7:8     1::3:4:5:6:7:8   1::8\n  |(?:(?:[0-9a-f]{1,4}:){1,2}(?::[0-9a-f]{1,4}){1,5})   # 1::4:5:6:7:8       1:2::4:5:6:7:8   1:2::8\n  |(?:(?:[0-9a-f]{1,4}:){1,3}(?::[0-9a-f]{1,4}){1,4})   # 1::5:6:7:8         1:2:3::5:6:7:8   1:2:3::8\n  |(?:(?:[0-9a-f]{1,4}:){1,4}(?::[0-9a-f]{1,4}){1,3})   # 1::6:7:8           1:2:3:4::6:7:8   1:2:3:4::8\n  |(?:(?:[0-9a-f]{1,4}:){1,5}(?::[0-9a-f]{1,4}){1,2})   # 1::7:8             1:2:3:4:5::7:8   1:2:3:4:5::8\n  |(?:(?:[0-9a-f]{1,4}:){1,6}   :[0-9a-f]{1,4})         # 1::8               1:2:3:4:5:6::8   1:2:3:4:5:6::8\n  |(?:(?:[0-9a-f]{1,4}:){1,7}   :)                      # 1::                                 1:2:3:4:5:6:7::\n  |(?::(?:(?::[0-9a-f]{1,4}){1,7}|:))                   # ::2:3:4:5:6:7:8    ::2:3:4:5:6:7:8  ::8       ::\n)(?![0-9A-Za-z:])".to_string());
    v.insert("zero_to_255".to_string(), "(?:(?:25[0-5])|(?:2[0-4][0-9])|(?:1[0-9][0-9])|(?:[1-9][0-9])|[0-9])".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 7014 });
    v.insert("string-double-quote".to_string(), ContextId { index: 7020 });
    v.insert("#anon_email-addresses_0".to_string(), ContextId { index: 6970 });
    v.insert("#anon_headers_0".to_string(), ContextId { index: 6975 });
    v.insert("body-content-default".to_string(), ContextId { index: 6985 });
    v.insert("#anon_body-content-default_0".to_string(), ContextId { index: 6962 });
    v.insert("encoded-words".to_string(), ContextId { index: 6993 });
    v.insert("pop-before-boundary".to_string(), ContextId { index: 7018 });
    v.insert("header-value".to_string(), ContextId { index: 7005 });
    v.insert("#anon_header-comment_0".to_string(), ContextId { index: 6973 });
    v.insert("expect-body-instructions-headers".to_string(), ContextId { index: 6996 });
    v.insert("email-addresses".to_string(), ContextId { index: 6992 });
    v.insert("#anon_body-content-html-quoted-printable_0".to_string(), ContextId { index: 6966 });
    v.insert("header-sub-values".to_string(), ContextId { index: 7004 });
    v.insert("uris".to_string(), ContextId { index: 7022 });
    v.insert("expect-body-instructions-html-noencoding".to_string(), ContextId { index: 6997 });
    v.insert("#anon_body-content-default_2".to_string(), ContextId { index: 6964 });
    v.insert("common".to_string(), ContextId { index: 6991 });
    v.insert("in-timestamp-year".to_string(), ContextId { index: 7011 });
    v.insert("in-timestamp-zone".to_string(), ContextId { index: 7012 });
    v.insert("in-timestamp-month".to_string(), ContextId { index: 7009 });
    v.insert("body-content-base64".to_string(), ContextId { index: 6984 });
    v.insert("body-content-quoted-printable".to_string(), ContextId { index: 6990 });
    v.insert("#anon_header-value_0".to_string(), ContextId { index: 6974 });
    v.insert("body-content-html".to_string(), ContextId { index: 6987 });
    v.insert("#anon_body-content-html_0".to_string(), ContextId { index: 6968 });
    v.insert("#anon_body-content-default_1".to_string(), ContextId { index: 6963 });
    v.insert("expect-body-instructions-quoted-printable".to_string(), ContextId { index: 7000 });
    v.insert("in-timestamp-time-of-day".to_string(), ContextId { index: 7010 });
    v.insert("#anon_uris_0".to_string(), ContextId { index: 6978 });
    v.insert("become-expect-base64".to_string(), ContextId { index: 6982 });
    v.insert("maybe-timestamp-day".to_string(), ContextId { index: 7015 });
    v.insert("expect-body-instructions-image".to_string(), ContextId { index: 6999 });
    v.insert("mime-type".to_string(), ContextId { index: 7016 });
    v.insert("body-content-html-quoted-printable".to_string(), ContextId { index: 6988 });
    v.insert("plaintext-blockquote".to_string(), ContextId { index: 7017 });
    v.insert("__main".to_string(), ContextId { index: 6979 });
    v.insert("expect-body-instructions-base64".to_string(), ContextId { index: 6995 });
    v.insert("html-with-quoted-printable".to_string(), ContextId { index: 7007 });
    v.insert("#anon_body-content-html-quoted-printable_1".to_string(), ContextId { index: 6967 });
    v.insert("header-comment".to_string(), ContextId { index: 7003 });
    v.insert("generic-header-kvp".to_string(), ContextId { index: 7001 });
    v.insert("ip-addresses".to_string(), ContextId { index: 7013 });
    v.insert("header-block".to_string(), ContextId { index: 7002 });
    v.insert("__start".to_string(), ContextId { index: 6980 });
    v.insert("timestamps".to_string(), ContextId { index: 7021 });
    v.insert("quoted-printable".to_string(), ContextId { index: 7019 });
    v.insert("headers".to_string(), ContextId { index: 7006 });
    v.insert("expect-body-instructions-html-quoted-printable".to_string(), ContextId { index: 6998 });
    v.insert("#anon_encoded-words_1".to_string(), ContextId { index: 6972 });
    v.insert("#anon_string-double-quote_0".to_string(), ContextId { index: 6977 });
    v.insert("expect-body-instructions".to_string(), ContextId { index: 6994 });
    v.insert("ignore-qp-newline-pop-nonws".to_string(), ContextId { index: 7008 });
    v.insert("body-content-image".to_string(), ContextId { index: 6989 });
    v.insert("#anon_html-with-quoted-printable_0".to_string(), ContextId { index: 6976 });
    v.insert("#anon_body-content-default_3".to_string(), ContextId { index: 6965 });
    v.insert("base64-line".to_string(), ContextId { index: 6981 });
    v.insert("body-content-headers".to_string(), ContextId { index: 6986 });
    v.insert("#anon_encoded-words_0".to_string(), ContextId { index: 6971 });
    v.insert("body-block".to_string(), ContextId { index: 6983 });
    v.insert("#anon_body-content-html_1".to_string(), ContextId { index: 6969 });
    v
  }
} }