
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 51510711032873123,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 51510711032873123,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 10250 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  AcceptEnv|AddressFamily|\n  Allow(?:AgentForwarding|Groups|StreamLocalForwarding|TcpForwarding|Users)|\n  AuthenticationMethods|\n  Authorized(?:Keys|Principals)(?:Command|CommandUser|File)|\n  Banner|\n  CASignatureAlgorithms|ChallengeResponseAuthentication|ChrootDirectory|\n  Ciphers|ClientAliveCountMax|ClientAliveInterval|Compression|\n  DenyGroups|DenyUsers|DisableForwarding|\n  ExposeAuthInfo|\n  FingerprintHash|ForceCommand|\n  GatewayPorts|GSSAPIAuthentication|GSSAPICleanupCredentials|GSSAPIStrictAcceptorCheck|\n  Hostbased(?:AcceptedKeyTypes|Authentication|UsesNameFromPacketOnly)|\n  HostCertificate|HostKey|HostKeyAgent|HostKeyAlgorithms|\n  IgnoreRhosts|IgnoreUserKnownHosts|IPQoS|\n  KbdInteractiveAuthentication|\n  Kerberos(?:Authentication|GetAFSToken|OrLocalPasswd|TicketCleanup)|\n  KexAlgorithms|KeyRegenerationInterval|\n  ListenAddress|LoginGraceTime|LogLevel|\n  MACs|Match|MaxAuthTries|MaxSessions|MaxStartups|\n  PasswordAuthentication|\n  Permit(?:EmptyPasswords|Listen|Open|RootLogin|TTY|Tunnel|UserEnvironment|UserRC)|\n  PidFile|Port|PrintLastLog|PrintMotd|Protocol|PubkeyAcceptedKeyTypes|PubkeyAuthentication|\n  RekeyLimit|RevokedKeys|RDomain|RhostsRSAAuthentication|RSAAuthentication|\n  ServerKeyBits|SetEnv|ShowPatchLevel|StreamLocalBindMask|StreamLocalBindUnlink|\n  StrictModes|Subsystem|SyslogFacility|\n  TCPKeepAlive|\n  UseDNS|UseLogin|UsePAM|UsePrivilegeSeparation|\n  VersionAddendum|\n  X11DisplayOffset|X11Forwarding|X11UseLocalhost|XAuthLocation\n)\\b"),
      scope: vec![
        Scope {
            a: 46444174328135843,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }