
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
      regex: Regex::new("(?=\\b(?x:\n  # control keywords\n  default|else|elsif|given|if|unless|when|break|caller|continue|die|\n  do|dump|exit|goto|last|next|redo|return|wait|for|foreach|until|while|\n  # declaration keywords\n  package|require|use|no|sub|format|local|my|our|state|\n  # word operators\n  and|or|xor|as|cmp|eq|gt|ge|lt|le|ne|not|x|\n  # quoted like functions (are handled like keywords)\n  m|q|qq|qr|qw|qx|s|tr|y\n)\\b|\\b(?x:\n  abs|accept|alarm|atan2|bind|binmode|bless|chdir|chmod|\n  chomp|chop|chown|chr|chroot|close|closedir|connect|cos|crypt|\n  dbmclose|dbmopen|defined|delete|each|endgrent|endhostent|\n  endnetent|endprotoent|endpwent|endservent|eof|eval|evalbytes|exec|\n  exists|exp|fc|fcntl|fileno|flock|fork|formline|getc|\n  getgrent|getgrgid|getgrnam|gethostbyaddr|gethostbyname|gethostent|\n  getlogin|getnetbyaddr|getnetbyname|getnetent|getpeername|getpgrp|\n  getppid|getpriority|getprotobyname|getprotobynumber|getprotoent|\n  getpwent|getpwnam|getpwuid|getservbyname|getservbyport|getservent|\n  getsockname|getsockopt|glob|gmtime|grep|hex|index|int|ioctl|join|\n  keys|kill|lc|lcfirst|length|link|listen|localtime|lock|log|\n  lstat|map|mkdir|msgctl|msgget|msgrcv|msgsnd|oct|open|opendir|ord|\n  pack|pipe|pop|pos|print|printf|prototype|push|quotemeta|\n  rand|read|readdir|readline|readlink|readpipe|recv|ref|rename|\n  reset|reverse|rewinddir|rindex|rmdir|say|scalar|seek|seekdir|select|\n  semctl|semget|semop|send|setgrent|sethostent|setnetent|setpgrp|\n  setpriority|setprotoent|setpwent|setservent|setsockopt|shift|shmctl|\n  shmget|shmread|shmwrite|shutdown|sin|sleep|socket|socketpair|sort|\n  splice|split|sprintf|sqrt|srand|stat|study|substr|symlink|syscall|\n  sysopen|sysread|sysseek|system|syswrite|tell|telldir|tie|tied|time|\n  times|truncate|uc|ucfirst|umask|undef|unlink|unpack|unshift|untie|\n  utime|values|vec|wait|waitpid|wantarray|warn|write\n)\\b(?!::))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[_\\p{L}]\\w*\\b"),
      scope: vec![
        Scope {
            a: 49259637051817984,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4291 })),
]
} }