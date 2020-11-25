
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
      regex: Regex::new("\\b(?i:echo)\\b"),
      scope: vec![
        Scope {
            a: 52637177855148032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 168 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 194 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:aciniupd|adprep|append|arp|assoc|at|atmadm|attrib|autofail|backup|basica|bcdedit|bootcfg|break|cacls|cd|cdburn|certreq|certutil|change logon|change port|change user|change|chcp|chdir|chglogon|chgport|chgusr|chkdsk|chkntfs|choice|cipher|cleanmgr|clip|cls|cluadmin|cluster|cmd|cmdkey|cmstp|color|comp|compact|control|convert|copy|cprofile|cscript|csvde|ctty|date|dcgpofix|debug|defrag|del|deltree|devcon|dfscmd|dhcploc|diantz|dir|diskcomp|diskcopy|diskpart|doskey|dpath|driverquery|dsadd|dsget|dsmod|dsmove|dsquery|dsrm|dvdburn|edit|edlin|endlocal|epal|erase|eventcreate|eventtriggers|evntcmd|exe2bin|expand|explorer|extract|fastopen|fc|fdisk|filever|find|findramd|findstr|finger|flattemp|forcedos|forfiles|format|freedisk|fsutil|ftp|ftype|getmac|gettype|gpresult|gpupdate|graftabl|gwbasic|help|helpctr|hostname|icacls|iexpress|ifconfig|iisreset|inuse|ipconfig|ipxroute|irftp|jt|keyb|label|ldifde|lfnfor|lh|loadhigh|lock|lodctr|logman|logoff|lpq|lpr|macfile|makecab|md|mem|mkdir|mklink|mmc|mode|more|mountvol|move|mrinfo|msd|msg|msiexec|msinfo32|mstsc|mtrace|nbtstat|net accounts|net computer|net config|net continue|net file|net group|net help|net helpmsg|net localgroup|net name|net pause|net print|net send|net session|net share|net start|net statistics|net stop|net time|net use|net user|net view|net|netdom|netsh|netstat|nlb|nlbmgr|nltest|notepad|nslookup|ntbackup|ntcmdprompt|ntdsutil|ntsd|ocsetup|odbcconf|openfiles|path|pathping|pause|pbadmin|pentnt|perfmon|ping|ping6|pkgmgr|popd|powercfg|print|prompt|pushd|qappsrv|qbasic|qchain|qfarm|qprocess|qserver|query process|query session|query termserver|query user|query|quser|qwinsta|rasdial|rasphone|rcp|rd|readline|recimg|recover|reg|regedit|regedt32|regini|register|regsvr32|relog|rem|ren|rename|replace|reset session|reset|restore|rexec|risetup|rmdir|robocopy|route|rsh|rsm|rss|runas|rundll|rundll32|sc|schtasks|secedit|set|setlocal|setx|sfc|shadow|shift|shutdown|sort|start|subinacl|subst|sysocmgr|systeminfo|takeown|tapicfg|taskkill|tasklist|tcmsetup|tftp|time|timeout|title|tracerpt|tracert|tracert6|tracerte|tree|truename|tscon|tsdiscon|tsecimp|tskill|tsprof|tsshutdn|type|typeperf|tzchange|unlock|unlodctr|ver|verify|vhdmount|vol|vssadmin|w32tm|waitfor|where|whoami|winmsd|winmsdp|winnt|winnt32|winpop|winsat|wlbs|wmic|wscript|wupdmgr|xcacls|xcopy)\\b"),
      scope: vec![
        Scope {
            a: 52637177855148032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }