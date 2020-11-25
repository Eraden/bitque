
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Groovy".to_string(),
  file_extensions: vec!["groovy".to_string(),"gvy".to_string(),"gradle".to_string(),"Jenkinsfile".to_string()],
  scope: Scope { a: 844566664052736, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("unicode_letter".to_string(), "(?:(?xi)\n  # Valid unicode letters according to:\n  # http://groovy-lang.org/syntax.html#_normal_identifiers\n  #   Literal Unicode         Escaped Unicode\n      [\\x{00C0}-\\x{00D6}]  |  \\\\u00C[0-9A-F] | \\\\u00D[0-6]\n    | [\\x{00D8}-\\x{00F6}]  |  \\\\u00D[89A-F]  | \\\\u00E[0-9A-F] | \\\\u00F[0-6]\n    | [\\x{00F8}-\\x{00FF}]  |  \\\\u00F[89A-F]\n    | [\\x{0100}-\\x{FFFE}]  |  \\\\u0[1-9A-F][0-9A-F]{2} | \\\\u(?!FFFF)[1-9A-F][0-9A-F]{3}\n)".to_string());
    v.insert("single_dollar_interpolation_identifier".to_string(), "(?:{{unicode_letter}}|[a-zA-Z_])(?:{{unicode_letter}}|[a-zA-Z0-9_])*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_method-declaration-remainder_1".to_string(), ContextId { index: 1995 });
    v.insert("__start".to_string(), ContextId { index: 2012 });
    v.insert("keyword-language".to_string(), ContextId { index: 2024 });
    v.insert("storage-types".to_string(), ContextId { index: 2035 });
    v.insert("string-escape-sequences".to_string(), ContextId { index: 2037 });
    v.insert("#anon_string-dollar-slashy_0".to_string(), ContextId { index: 2002 });
    v.insert("constants".to_string(), ContextId { index: 2018 });
    v.insert("#anon_method-declaration-remainder_0".to_string(), ContextId { index: 1994 });
    v.insert("keyword-operator".to_string(), ContextId { index: 2025 });
    v.insert("#anon_escaped-end-of-line_0".to_string(), ContextId { index: 1989 });
    v.insert("__main".to_string(), ContextId { index: 2011 });
    v.insert("block".to_string(), ContextId { index: 2013 });
    v.insert("#anon_classes_1".to_string(), ContextId { index: 1987 });
    v.insert("variables".to_string(), ContextId { index: 2047 });
    v.insert("#anon_methods_1".to_string(), ContextId { index: 1997 });
    v.insert("numbers".to_string(), ContextId { index: 2031 });
    v.insert("string-quoted-double".to_string(), ContextId { index: 2038 });
    v.insert("class-object".to_string(), ContextId { index: 2014 });
    v.insert("storage-modifiers".to_string(), ContextId { index: 2034 });
    v.insert("string-quoted-triple-double".to_string(), ContextId { index: 2040 });
    v.insert("unicode-escape-sequence".to_string(), ContextId { index: 2045 });
    v.insert("values".to_string(), ContextId { index: 2046 });
    v.insert("map-keys".to_string(), ContextId { index: 2027 });
    v.insert("groovy-code-minus-map-keys".to_string(), ContextId { index: 2022 });
    v.insert("#anon_string-quoted-triple-double_1".to_string(), ContextId { index: 2008 });
    v.insert("comments".to_string(), ContextId { index: 2017 });
    v.insert("methods".to_string(), ContextId { index: 2030 });
    v.insert("#anon_regexp_1".to_string(), ContextId { index: 1999 });
    v.insert("structures".to_string(), ContextId { index: 2043 });
    v.insert("regexp".to_string(), ContextId { index: 2032 });
    v.insert("classes".to_string(), ContextId { index: 2015 });
    v.insert("string-dollar-slashy".to_string(), ContextId { index: 2036 });
    v.insert("#anon_string-quoted-double_0".to_string(), ContextId { index: 2004 });
    v.insert("string-quoted-triple-single".to_string(), ContextId { index: 2041 });
    v.insert("#anon_string-quoted-double_1".to_string(), ContextId { index: 2005 });
    v.insert("#anon_block_0".to_string(), ContextId { index: 1985 });
    v.insert("#anon_storage-types_0".to_string(), ContextId { index: 2001 });
    v.insert("#anon_classes_0".to_string(), ContextId { index: 1986 });
    v.insert("#anon_keyword-language_0".to_string(), ContextId { index: 1990 });
    v.insert("#anon_keyword-language_1".to_string(), ContextId { index: 1991 });
    v.insert("#anon_structures_0".to_string(), ContextId { index: 2010 });
    v.insert("method-call".to_string(), ContextId { index: 2028 });
    v.insert("comment-block".to_string(), ContextId { index: 2016 });
    v.insert("groovy".to_string(), ContextId { index: 2020 });
    v.insert("strings".to_string(), ContextId { index: 2042 });
    v.insert("string-quoted-single".to_string(), ContextId { index: 2039 });
    v.insert("#anon_keyword-operator_0".to_string(), ContextId { index: 1992 });
    v.insert("#anon_regexp_0".to_string(), ContextId { index: 1998 });
    v.insert("groovy-code".to_string(), ContextId { index: 2021 });
    v.insert("#anon_string-quoted-triple-single_0".to_string(), ContextId { index: 2009 });
    v.insert("main".to_string(), ContextId { index: 2026 });
    v.insert("#anon_comment-block_0".to_string(), ContextId { index: 1988 });
    v.insert("#anon_methods_0".to_string(), ContextId { index: 1996 });
    v.insert("#anon_single-dollar-string-interpolation_0".to_string(), ContextId { index: 2000 });
    v.insert("#anon_string-dollar-slashy_1".to_string(), ContextId { index: 2003 });
    v.insert("#anon_string-quoted-single_0".to_string(), ContextId { index: 2006 });
    v.insert("keyword".to_string(), ContextId { index: 2023 });
    v.insert("#anon_string-quoted-triple-double_0".to_string(), ContextId { index: 2007 });
    v.insert("single-dollar-string-interpolation".to_string(), ContextId { index: 2033 });
    v.insert("support-functions".to_string(), ContextId { index: 2044 });
    v.insert("#anon_method-call_0".to_string(), ContextId { index: 1993 });
    v.insert("method-declaration-remainder".to_string(), ContextId { index: 2029 });
    v.insert("escaped-end-of-line".to_string(), ContextId { index: 2019 });
    v
  }
} }