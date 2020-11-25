
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SSHD Config".to_string(),
  file_extensions: vec!["sshd_config".to_string()],
  scope: Scope { a: 845125009801216, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("all_options".to_string(), "\\b(?xi:\n  AcceptEnv|AddressFamily|\n  Allow(?:AgentForwarding|Groups|StreamLocalForwarding|TcpForwarding|Users)|\n  AuthenticationMethods|\n  Authorized(?:Keys|Principals)(?:Command|CommandUser|File)|\n  Banner|\n  CASignatureAlgorithms|ChallengeResponseAuthentication|ChrootDirectory|\n  Ciphers|ClientAliveCountMax|ClientAliveInterval|Compression|\n  DenyGroups|DenyUsers|DisableForwarding|\n  ExposeAuthInfo|\n  FingerprintHash|ForceCommand|\n  GatewayPorts|GSSAPIAuthentication|GSSAPICleanupCredentials|GSSAPIStrictAcceptorCheck|\n  Hostbased(?:AcceptedKeyTypes|Authentication|UsesNameFromPacketOnly)|\n  HostCertificate|HostKey|HostKeyAgent|HostKeyAlgorithms|\n  IgnoreRhosts|IgnoreUserKnownHosts|IPQoS|\n  KbdInteractiveAuthentication|\n  Kerberos(?:Authentication|GetAFSToken|OrLocalPasswd|TicketCleanup)|\n  KexAlgorithms|KeyRegenerationInterval|\n  ListenAddress|LoginGraceTime|LogLevel|\n  MACs|Match|MaxAuthTries|MaxSessions|MaxStartups|\n  PasswordAuthentication|\n  Permit(?:EmptyPasswords|Listen|Open|RootLogin|TTY|Tunnel|UserEnvironment|UserRC)|\n  PidFile|Port|PrintLastLog|PrintMotd|Protocol|PubkeyAcceptedKeyTypes|PubkeyAuthentication|\n  RekeyLimit|RevokedKeys|RDomain|RhostsRSAAuthentication|RSAAuthentication|\n  ServerKeyBits|SetEnv|ShowPatchLevel|StreamLocalBindMask|StreamLocalBindUnlink|\n  StrictModes|Subsystem|SyslogFacility|\n  TCPKeepAlive|\n  UseDNS|UseLogin|UsePAM|UsePrivilegeSeparation|\n  VersionAddendum|\n  X11DisplayOffset|X11Forwarding|X11UseLocalhost|XAuthLocation\n)\\b".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_comments_1".to_string(), ContextId { index: 10309 });
    v.insert("#anon_generic-option_2".to_string(), ContextId { index: 10313 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 10308 });
    v.insert("#anon_match-conditions_2".to_string(), ContextId { index: 10316 });
    v.insert("forcecommand".to_string(), ContextId { index: 10320 });
    v.insert("match".to_string(), ContextId { index: 10324 });
    v.insert("match-conditions".to_string(), ContextId { index: 10326 });
    v.insert("match-body".to_string(), ContextId { index: 10325 });
    v.insert("options".to_string(), ContextId { index: 10327 });
    v.insert("#anon_forcecommand_0".to_string(), ContextId { index: 10310 });
    v.insert("__start".to_string(), ContextId { index: 10318 });
    v.insert("#anon_match-conditions_0".to_string(), ContextId { index: 10314 });
    v.insert("#anon_generic-option_0".to_string(), ContextId { index: 10311 });
    v.insert("pop-before-match-option".to_string(), ContextId { index: 10328 });
    v.insert("pop-nl-as-value".to_string(), ContextId { index: 10330 });
    v.insert("comments".to_string(), ContextId { index: 10319 });
    v.insert("#anon_match-conditions_1".to_string(), ContextId { index: 10315 });
    v.insert("pop-before-next-match".to_string(), ContextId { index: 10329 });
    v.insert("string-patterns".to_string(), ContextId { index: 10331 });
    v.insert("tokens".to_string(), ContextId { index: 10332 });
    v.insert("#anon_generic-option_1".to_string(), ContextId { index: 10312 });
    v.insert("__main".to_string(), ContextId { index: 10317 });
    v.insert("generic-option-value".to_string(), ContextId { index: 10322 });
    v.insert("generic-option".to_string(), ContextId { index: 10321 });
    v.insert("main".to_string(), ContextId { index: 10323 });
    v
  }
} }