
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Go".to_string(),
  file_extensions: vec!["go".to_string()],
  scope: Scope { a: 844558074118144, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("keyword".to_string(), "\\b(?:break|case|chan|const|continue|default|defer|else|fallthrough|for|func|go|goto|if|import|interface|map|package|range|return|select|struct|switch|type|var)\\b".to_string());
    v.insert("char_escape".to_string(), "\\\\x\\h{2}|\\\\u\\h{4}|\\\\U\\h{8}|\\\\[0-7]{3}|\\\\.".to_string());
    v.insert("bin_digits".to_string(), "(?:_?[01]+(?:_[01]+)*)".to_string());
    v.insert("dec_exponent".to_string(), "(?:[eE][-+]??{{dec_digits}})".to_string());
    v.insert("ohdigits".to_string(), "_?(?:\\h*(?:_\\h+)*)".to_string());
    v.insert("hdigits".to_string(), "_?(?:\\h+(?:_\\h+)*)".to_string());
    v.insert("oct_digits".to_string(), "(?:_?[0-7]+(?:_[0-7]+)*)".to_string());
    v.insert("bdigits".to_string(), "_?(?:[01]+(?:_[01]+)*)".to_string());
    v.insert("hex_digits".to_string(), "(?:_?\\h+(?:_\\h+)*)".to_string());
    v.insert("ident".to_string(), "\\b(?!{{keyword}})[[:alpha:]_][[:alnum:]_]*\\b".to_string());
    v.insert("dec_digits".to_string(), "(?:\\d+(?:_\\d+)*)".to_string());
    v.insert("hex_exponent".to_string(), "(?:[pP][-+]??{{dec_digits}})".to_string());
    v.insert("predeclared_type".to_string(), "\\b(?:bool|byte|complex64|complex128|error|float32|float64|int|int8|int16|int32|int64|rune|string|uint|uint8|uint16|uint32|uint64|uintptr)\\b".to_string());
    v.insert("inline_comment".to_string(), "/[*](?:[^*]|[*](?!/))*[*]/".to_string());
    v.insert("ddigits".to_string(), "(?:\\d+(?:_\\d+)*)".to_string());
    v.insert("odigits".to_string(), "_?(?:[0-7]+(?:_[0-7]+)*)".to_string());
    v.insert("predeclared_func".to_string(), "\\b(?:append|cap|close|complex|copy|delete|imag|len|make|new|panic|print|println|real|recover)\\b".to_string());
    v.insert("noise".to_string(), "(?:\\s|{{inline_comment}})*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("match-binary-integer".to_string(), ContextId { index: 1881 });
    v.insert("#anon_pop-struct_2".to_string(), ContextId { index: 1873 });
    v.insert("match-keyword-fallthrough".to_string(), ContextId { index: 1905 });
    v.insert("match-octal-integer".to_string(), ContextId { index: 1924 });
    v.insert("match-predeclared-constants".to_string(), ContextId { index: 1927 });
    v.insert("#anon_pop-func-parameter-list_0".to_string(), ContextId { index: 1863 });
    v.insert("#anon_match-interpreted-string_0".to_string(), ContextId { index: 1844 });
    v.insert("pop-const-type-and-or-assignment".to_string(), ContextId { index: 1944 });
    v.insert("#anon_pop-map_0".to_string(), ContextId { index: 1867 });
    v.insert("main".to_string(), ContextId { index: 1879 });
    v.insert("match-template-string".to_string(), ContextId { index: 1936 });
    v.insert("#anon_match-keyword-type_2".to_string(), ContextId { index: 1852 });
    v.insert("pop-interface".to_string(), ContextId { index: 1949 });
    v.insert("pop-member".to_string(), ContextId { index: 1952 });
    v.insert("#anon_match-raw-string_0".to_string(), ContextId { index: 1857 });
    v.insert("match-keyword-else".to_string(), ContextId { index: 1904 });
    v.insert("match-strings".to_string(), ContextId { index: 1935 });
    v.insert("pop-type".to_string(), ContextId { index: 1960 });
    v.insert("match-any".to_string(), ContextId { index: 1880 });
    v.insert("match-keyword-map".to_string(), ContextId { index: 1913 });
    v.insert("match-keyword-break".to_string(), ContextId { index: 1897 });
    v.insert("match-runes".to_string(), ContextId { index: 1930 });
    v.insert("match-call-or-cast".to_string(), ContextId { index: 1884 });
    v.insert("#anon_pop-var-expressions_0".to_string(), ContextId { index: 1876 });
    v.insert("#anon_match-keyword-type_0".to_string(), ContextId { index: 1850 });
    v.insert("match-brackets".to_string(), ContextId { index: 1883 });
    v.insert("match-identifiers".to_string(), ContextId { index: 1893 });
    v.insert("#anon_match-keyword-func_1".to_string(), ContextId { index: 1848 });
    v.insert("match-keyword-chan".to_string(), ContextId { index: 1899 });
    v.insert("pop-func-return-signature".to_string(), ContextId { index: 1947 });
    v.insert("#anon_match-keyword-func_0".to_string(), ContextId { index: 1847 });
    v.insert("pop-arguments-starting-with-type".to_string(), ContextId { index: 1938 });
    v.insert("pop-func-signature".to_string(), ContextId { index: 1948 });
    v.insert("pop-var-assignment-or-terminate".to_string(), ContextId { index: 1965 });
    v.insert("pop-type-nested-in-parens".to_string(), ContextId { index: 1964 });
    v.insert("match-decimal-integer".to_string(), ContextId { index: 1888 });
    v.insert("#anon_match-keyword-var_0".to_string(), ContextId { index: 1854 });
    v.insert("match-keyword-for".to_string(), ContextId { index: 1906 });
    v.insert("match-keyword-go".to_string(), ContextId { index: 1908 });
    v.insert("#anon_match-keyword-func_2".to_string(), ContextId { index: 1849 });
    v.insert("match-star".to_string(), ContextId { index: 1934 });
    v.insert("match-braces".to_string(), ContextId { index: 1882 });
    v.insert("pop-chan".to_string(), ContextId { index: 1941 });
    v.insert("#anon_pop-map_2".to_string(), ContextId { index: 1869 });
    v.insert("match-literals".to_string(), ContextId { index: 1923 });
    v.insert("match-colon".to_string(), ContextId { index: 1885 });
    v.insert("pop-on-semicolon".to_string(), ContextId { index: 1954 });
    v.insert("#anon_match-keyword-var_1".to_string(), ContextId { index: 1855 });
    v.insert("pop-struct".to_string(), ContextId { index: 1959 });
    v.insert("pop-var-expressions".to_string(), ContextId { index: 1966 });
    v.insert("match-short-variable-declarations".to_string(), ContextId { index: 1933 });
    v.insert("match-comma".to_string(), ContextId { index: 1886 });
    v.insert("match-operators".to_string(), ContextId { index: 1925 });
    v.insert("pop-before-nonblank".to_string(), ContextId { index: 1939 });
    v.insert("match-ellipsis".to_string(), ContextId { index: 1889 });
    v.insert("pop-const-expressions".to_string(), ContextId { index: 1943 });
    v.insert("pop-parameter-list-unnamed".to_string(), ContextId { index: 1957 });
    v.insert("pop-type-identifier".to_string(), ContextId { index: 1963 });
    v.insert("match-keyword-range".to_string(), ContextId { index: 1915 });
    v.insert("#anon_pop-interface_1".to_string(), ContextId { index: 1865 });
    v.insert("match-keyword-struct".to_string(), ContextId { index: 1918 });
    v.insert("match-hex-integer".to_string(), ContextId { index: 1892 });
    v.insert("pop-parameter-type".to_string(), ContextId { index: 1958 });
    v.insert("pop-type-assertion".to_string(), ContextId { index: 1962 });
    v.insert("match-keyword-default".to_string(), ContextId { index: 1902 });
    v.insert("match-keyword-select".to_string(), ContextId { index: 1917 });
    v.insert("#anon_match-keyword-const_1".to_string(), ContextId { index: 1846 });
    v.insert("match-keyword-package".to_string(), ContextId { index: 1914 });
    v.insert("#anon_match-brackets_0".to_string(), ContextId { index: 1841 });
    v.insert("match-keyword-goto".to_string(), ContextId { index: 1909 });
    v.insert("__main".to_string(), ContextId { index: 1877 });
    v.insert("match-selector".to_string(), ContextId { index: 1931 });
    v.insert("#anon_pop-interface_2".to_string(), ContextId { index: 1866 });
    v.insert("match-imaginary".to_string(), ContextId { index: 1894 });
    v.insert("#anon_match-keyword-type_1".to_string(), ContextId { index: 1851 });
    v.insert("#anon_match-parens_0".to_string(), ContextId { index: 1856 });
    v.insert("pop-func-parameter-list".to_string(), ContextId { index: 1946 });
    v.insert("match-parens".to_string(), ContextId { index: 1926 });
    v.insert("match-keywords".to_string(), ContextId { index: 1922 });
    v.insert("#anon_pop-interface_0".to_string(), ContextId { index: 1864 });
    v.insert("match-floats".to_string(), ContextId { index: 1890 });
    v.insert("#anon_pop-const-expressions_0".to_string(), ContextId { index: 1862 });
    v.insert("match-comments".to_string(), ContextId { index: 1887 });
    v.insert("match-keyword-func".to_string(), ContextId { index: 1907 });
    v.insert("pop-var-type-and-or-assignment".to_string(), ContextId { index: 1967 });
    v.insert("match-keyword-continue".to_string(), ContextId { index: 1901 });
    v.insert("pop-call-or-cast".to_string(), ContextId { index: 1940 });
    v.insert("#anon_pop-struct_0".to_string(), ContextId { index: 1871 });
    v.insert("#anon_pop-struct_1".to_string(), ContextId { index: 1872 });
    v.insert("match-integers".to_string(), ContextId { index: 1895 });
    v.insert("__start".to_string(), ContextId { index: 1878 });
    v.insert("pop-type-alias-or-typedef".to_string(), ContextId { index: 1961 });
    v.insert("#anon_match-short-variable-declarations_0".to_string(), ContextId { index: 1859 });
    v.insert("#anon_match-keyword-type_3".to_string(), ContextId { index: 1853 });
    v.insert("match-interpreted-string".to_string(), ContextId { index: 1896 });
    v.insert("pop-parameter-list-named".to_string(), ContextId { index: 1956 });
    v.insert("#anon_match-selector_0".to_string(), ContextId { index: 1858 });
    v.insert("pop-line-comment".to_string(), ContextId { index: 1950 });
    v.insert("pop-named-type".to_string(), ContextId { index: 1953 });
    v.insert("#anon_pop-map_3".to_string(), ContextId { index: 1870 });
    v.insert("#anon_pop-map_1".to_string(), ContextId { index: 1868 });
    v.insert("match-punctuation".to_string(), ContextId { index: 1928 });
    v.insert("match-fmt".to_string(), ContextId { index: 1891 });
    v.insert("pop-map".to_string(), ContextId { index: 1951 });
    v.insert("#anon_pop-type-assertion_0".to_string(), ContextId { index: 1874 });
    v.insert("match-keyword-if".to_string(), ContextId { index: 1910 });
    v.insert("match-keyword-const".to_string(), ContextId { index: 1900 });
    v.insert("#anon_match-keyword-const_0".to_string(), ContextId { index: 1845 });
    v.insert("match-keyword-switch".to_string(), ContextId { index: 1919 });
    v.insert("#anon_match-comments_0".to_string(), ContextId { index: 1842 });
    v.insert("#anon_pop-arguments-starting-with-type_0".to_string(), ContextId { index: 1861 });
    v.insert("#anon_match-template-string_0".to_string(), ContextId { index: 1860 });
    v.insert("pop-on-terminator".to_string(), ContextId { index: 1955 });
    v.insert("match-keyword-return".to_string(), ContextId { index: 1916 });
    v.insert("#anon_match-braces_0".to_string(), ContextId { index: 1840 });
    v.insert("match-keyword-case".to_string(), ContextId { index: 1898 });
    v.insert("match-keyword-interface".to_string(), ContextId { index: 1912 });
    v.insert("match-tokens".to_string(), ContextId { index: 1937 });
    v.insert("pop-const-assignment-or-terminate".to_string(), ContextId { index: 1942 });
    v.insert("#anon_pop-type_0".to_string(), ContextId { index: 1875 });
    v.insert("match-raw-string".to_string(), ContextId { index: 1929 });
    v.insert("match-keyword-var".to_string(), ContextId { index: 1921 });
    v.insert("match-keyword-defer".to_string(), ContextId { index: 1903 });
    v.insert("match-semicolon".to_string(), ContextId { index: 1932 });
    v.insert("#anon_match-comments_1".to_string(), ContextId { index: 1843 });
    v.insert("pop-func-parameter-and-return-lists".to_string(), ContextId { index: 1945 });
    v.insert("match-keyword-import".to_string(), ContextId { index: 1911 });
    v.insert("match-keyword-type".to_string(), ContextId { index: 1920 });
    v
  }
} }