
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?x:\n  abs|accept|alarm|atan2|bind|binmode|bless|chdir|chmod|\n  chomp|chop|chown|chr|chroot|close|closedir|connect|cos|crypt|\n  dbmclose|dbmopen|defined|delete|each|endgrent|endhostent|\n  endnetent|endprotoent|endpwent|endservent|eof|eval|evalbytes|exec|\n  exists|exp|fc|fcntl|fileno|flock|fork|formline|getc|\n  getgrent|getgrgid|getgrnam|gethostbyaddr|gethostbyname|gethostent|\n  getlogin|getnetbyaddr|getnetbyname|getnetent|getpeername|getpgrp|\n  getppid|getpriority|getprotobyname|getprotobynumber|getprotoent|\n  getpwent|getpwnam|getpwuid|getservbyname|getservbyport|getservent|\n  getsockname|getsockopt|glob|gmtime|grep|hex|index|int|ioctl|join|\n  keys|kill|lc|lcfirst|length|link|listen|localtime|lock|log|\n  lstat|map|mkdir|msgctl|msgget|msgrcv|msgsnd|oct|open|opendir|ord|\n  pack|pipe|pop|pos|print|printf|prototype|push|quotemeta|\n  rand|read|readdir|readline|readlink|readpipe|recv|ref|rename|\n  reset|reverse|rewinddir|rindex|rmdir|say|scalar|seek|seekdir|select|\n  semctl|semget|semop|send|setgrent|sethostent|setnetent|setpgrp|\n  setpriority|setprotoent|setpwent|setservent|setsockopt|shift|shmctl|\n  shmget|shmread|shmwrite|shutdown|sin|sleep|socket|socketpair|sort|\n  splice|split|sprintf|sqrt|srand|stat|study|substr|symlink|syscall|\n  sysopen|sysread|sysseek|system|syswrite|tell|telldir|tie|tied|time|\n  times|truncate|uc|ucfirst|umask|undef|unlink|unpack|unshift|untie|\n  utime|values|vec|wait|waitpid|wantarray|warn|write\n)\\b(?!::)"),
      scope: vec![
        Scope {
            a: 46444882989744128,
            b: 0,
        },
        Scope {
            a: 61925255089553408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[_\\p{L}]\\w*\\b(?=\\s*(?:(?m:$)|[;#/(\'\"`$@%]|<<|(?!(?:and|or|xor|as|cmp|eq|gt|ge|lt|le|ne|not|x)\\b(?!::))\\w))"),
      scope: vec![
        Scope {
            a: 46444882989744128,
            b: 0,
        },
        Scope {
            a: 49258881137573888,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
]
} }