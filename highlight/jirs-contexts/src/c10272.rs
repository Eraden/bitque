
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
      regex: Regex::new("(?x:\n  ^\\s*((?i:\n    (?:Pubkey|HostBased|Password|ChallengeResponse|\n     KbdInteractive|(?:Rhosts)?RSA)\n      Authentication| # Auth\n    ForwardAgent|ForwardX11(?:Trusted)?|ClearAllForwardings|\n      ExitOnForwardFailure| # Fwds\n    BatchMode|CanonicalizeFallbackLocal|CheckHostIP|Compression|\n    EnableSSHKeySign|GatewayPorts|HashKnownHosts|IdentitiesOnly|\n    NoHostAuthenticationForLocalhost|PermitLocalCommand|ProxyUseFdpass|\n    StreamLocalBindUnlink|TCPKeepAlive|UseKeychain|UsePrivilegedPort|\n    VisualHostKey|\n    GSSAPI(?:Authentication|KeyExchange|DelegateCredentials|\n           RenewalForcesRekey|TrustDNS) # GSSAPI\n  ))\\b[ \\t]*(=)?\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244577,
            b: 0,
        },
        Scope {
            a: 52636787023085568,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130785,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10255 }),
    ]),
      with_prototype: None
    }),
]
} }