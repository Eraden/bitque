
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
        a: 844781412417536,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844781412417536,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s)*(#).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711037657206,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038803,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)(Proxy|ProxyMatch|IfVersion|Directory|DirectoryMatch|Files|FilesMatch|IfDefine|IfModule|Limit|LimitExcept|Location|LocationMatch|VirtualHost|Macro|If|Else|ElseIf)(\\s(.+?))?(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152915,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392079084847104,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 55452146670436352,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629324152915,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</)(Proxy|ProxyMatch|IfVersion|Directory|DirectoryMatch|Files|FilesMatch|IfDefine|IfModule|Limit|LimitExcept|Location|LocationMatch|VirtualHost|Macro|If|Else|ElseIf)(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152915,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392079084847104,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629324152915,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=(Rewrite(Rule|Cond)))\\s+(.+?)\\s+(.+?)((?m:$)|\\s)"),
      scope: vec![],
      captures: Some(vec![(3, vec![
        Scope {
            a: 55450759395999744,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 55457051523088384,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=RedirectMatch)(\\s+(\\d\\d\\d|permanent|temp|seeother|gone))?\\s+(.+?)\\s+((.+?)((?m:$)|\\s))?"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59397705492004864,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55450759395999744,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 55452451613114368,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=Redirect)(\\s+(\\d\\d\\d|permanent|temp|seeother|gone))?\\s+(.+?)\\s+((.+?)((?m:$)|\\s))?"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59397705492004864,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55452451613114368,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 55452451613114368,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=ScriptAliasMatch|AliasMatch)\\s+(.+?)\\s+((.+?)\\s)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55450759395999744,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55452451613114368,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=RedirectPermanent|RedirectTemp|ScriptAlias|Alias)\\s+(.+?)\\s+((.+?)((?m:$)|\\s))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55452451613114368,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55452451613114368,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AcceptPathInfo|AccessFileName|AddDefaultCharset|AddOutputFilterByType|AllowEncodedSlashes|AllowOverride|AuthName|AuthType|CGIMapExtension|ContentDigest|DefaultType|Define|DocumentRoot|EnableMMAP|EnableSendfile|ErrorDocument|ErrorLog|FileETag|ForceType|HostnameLookups|IdentityCheck|Include(Optional)?|KeepAlive|KeepAliveTimeout|LimitInternalRecursion|LimitRequestBody|LimitRequestFields|LimitRequestFieldSize|LimitRequestLine|LimitXMLRequestBody|LogLevel|MaxKeepAliveRequests|Mutex|NameVirtualHost|Options|Require|RLimitCPU|RLimitMEM|RLimitNPROC|Satisfy|ScriptInterpreterSource|ServerAdmin|ServerAlias|ServerName|ServerPath|ServerRoot|ServerSignature|ServerTokens|SetHandler|SetInputFilter|SetOutputFilter|Time(O|o)ut|TraceEnable|UseCanonicalName|Use|ErrorLogFormat|GlobalLog|PHPIniDir|SSLHonorCipherOrder|SSLCompression|SSLUseStapling|SSLStapling\\w+|SSLCARevocationCheck|SSLSRPVerifierFile|SSLSessionTickets|RequestReadTimeout|ProxyHTML\\w+|MaxRanges)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52640661078474752,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AcceptMutex|AssignUserID|BS2000Account|ChildPerUserID|CoreDumpDirectory|EnableExceptionHook|Group|Listen|ListenBacklog|LockFile|MaxClients|MaxConnectionsPerChild|MaxMemFree|MaxRequestsPerChild|MaxRequestsPerThread|MaxRequestWorkers|MaxSpareServers|MaxSpareThreads|MaxThreads|MaxThreadsPerChild|MinSpareServers|MinSpareThreads|NumServers|PidFile|ReceiveBufferSize|ScoreBoardFile|SendBufferSize|ServerLimit|StartServers|StartThreads|ThreadLimit|ThreadsPerChild|ThreadStackSize|User|Win32DisableAcceptEx)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642310345916416,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Allow|Deny|Order)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52637487097643008,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Action|Script)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642314640883712,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Alias|AliasMatch|Redirect|RedirectMatch|RedirectPermanent|RedirectTemp|ScriptAlias|ScriptAliasMatch)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52637749090648064,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AuthAuthoritative|AuthGroupFile|AuthUserFile|AuthBasicProvider|AuthBasicFake|AuthBasicAuthoritative|AuthBasicUseDigestAlgorithm)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642318935851008,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Anonymous|Anonymous_Authoritative|Anonymous_LogEmail|Anonymous_MustGiveEmail|Anonymous_NoUserID|Anonymous_VerifyEmail)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642323230818304,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AuthDBMAuthoritative|AuthDBMGroupFile|AuthDBMType|AuthDBMUserFile)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642327525785600,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AuthDigestAlgorithm|AuthDigestDomain|AuthDigestFile|AuthDigestGroupFile|AuthDigestNcCheck|AuthDigestNonceFormat|AuthDigestNonceLifetime|AuthDigestQop|AuthDigestShmemSize|AuthDigestProvider)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642331820752896,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AuthLDAPAuthoritative|AuthLDAPBindDN|AuthLDAPBindPassword|AuthLDAPCharsetConfig|AuthLDAPCompareDNOnServer|AuthLDAPDereferenceAliases|AuthLDAPEnabled|AuthLDAPFrontPageHack|AuthLDAPGroupAttribute|AuthLDAPGroupAttributeIsDN|AuthLDAPRemoteUserIsDN|AuthLDAPUrl)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642336115720192,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AddAlt|AddAltByEncoding|AddAltByType|AddDescription|AddIcon|AddIconByEncoding|AddIconByType|DefaultIcon|HeaderName|IndexIgnore|IndexOptions|IndexOrderDefault|IndexStyleSheet|IndexHeadInsert|ReadmeName)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642340410687488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(BalancerMember|BalancerGrowth|BalancerPersist|BalancerInherit)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638058328293376,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(CacheDefaultExpire|CacheDisable|CacheEnable|CacheForceCompletion|CacheIgnoreCacheControl|CacheIgnoreHeaders|CacheIgnoreNoLastMod|CacheLastModifiedFactor|CacheMaxExpire)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642344705654784,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(MetaDir|MetaFiles|MetaSuffix)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642349000622080,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ScriptLog|ScriptLogBuffer|ScriptLogLength)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642353295589376,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ScriptLog|ScriptLogBuffer|ScriptLogLength|ScriptSock)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642357590556672,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(CharsetDefault|CharsetOptions|CharsetSourceEnc)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642361885523968,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Dav|DavDepthInfinity|DavMinTimeout|DavLockDB)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642366180491264,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(DeflateBufferSize|DeflateCompressionLevel|DeflateFilterNote|DeflateMemLevel|DeflateWindowSize)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642370475458560,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(DirectoryIndex|DirectorySlash|FallbackResource)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52640811402330112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(CacheDirLength|CacheDirLevels|CacheExpiryCheck|CacheGcClean|CacheGcDaily|CacheGcInterval|CacheGcMemUsage|CacheGcUnused|CacheMaxFileSize|CacheMinFileSize|CacheRoot|CacheSize|CacheTimeMargin)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642374770425856,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(DumpIOInput|DumpIOOutput)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642379065393152,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(PassEnv|SetEnv|UnsetEnv)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636241557127168,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ExpiresActive|ExpiresByType|ExpiresDefault)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642383360360448,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ExtFilterDefine|ExtFilterOptions)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642387655327744,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(CacheFile|MMapFile)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642391950295040,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AddOutputFilterByType|FilterChain|FilterDeclare|FilterProtocol|FilterProvider|FilterTrace)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638058328293376,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Header|RequestHeader)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642396245262336,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ImapBase|ImapDefault|ImapMenu)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642400540229632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(SSIEndTag|SSIErrorMsg|SSIStartTag|SSITimeFormat|SSIUndefinedEcho|XBitHack)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52637770565484544,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ISAPIAppendLogToErrors|ISAPIAppendLogToQuery|ISAPICacheFile|ISAPIFakeAsync|ISAPILogNotSupported|ISAPIReadAheadBuffer)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642404835196928,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(LDAPCacheEntries|LDAPCacheTTL|LDAPConnectionTimeout|LDAPOpCacheEntries|LDAPOpCacheTTL|LDAPSharedCacheFile|LDAPSharedCacheSize|LDAPTrustedCA|LDAPTrustedCAType)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52640884416774144,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(BufferedLogs|CookieLog|CustomLog|LogFormat|TransferLog|ForensicLog)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52635940909416448,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(MCacheMaxObjectCount|MCacheMaxObjectSize|MCacheMaxStreamingBuffer|MCacheMinObjectSize|MCacheRemovalAlgorithm|MCacheSize)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642409130164224,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AddCharset|AddEncoding|AddHandler|AddInputFilter|AddLanguage|AddOutputFilter|AddType|DefaultLanguage|ModMimeUsePathInfo|MultiviewsMatch|RemoveCharset|RemoveEncoding|RemoveHandler|RemoveInputFilter|RemoveLanguage|RemoveOutputFilter|RemoveType|TypesConfig)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642413425131520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ProtocolEcho|Example|AddModuleInfo|MimeMagicFile|CheckSpelling|ExtendedStatus|SuexecUserGroup|UserDir)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52641464237359104,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(CacheNegotiatedDocs|ForceLanguagePriority|LanguagePriority)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642417720098816,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(NWSSLTrustedCerts|NWSSLUpgradeable|SecureListen)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642422015066112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AllowCONNECT|NoProxy|ProxyBadHeader|ProxyBlock|ProxyDomain|ProxyErrorOverride|ProxyFtpDirCharset|ProxyIOBufferSize|ProxyMaxForwards|ProxyPass|ProxyPassMatch|ProxyPassReverse|ProxyPreserveHost|ProxyReceiveBufferSize|ProxyRemote|ProxyRemoteMatch|ProxyRequests|ProxyTimeout|ProxyVia)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642426310033408,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(RewriteBase|RewriteCond|RewriteEngine|RewriteLock|RewriteLog|RewriteLogLevel|RewriteMap|RewriteOptions|RewriteRule)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642430605000704,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(BrowserMatch|BrowserMatchNoCase|SetEnvIf|SetEnvIfNoCase)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642434899968000,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(LoadFile|LoadModule)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642439194935296,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(SSLCACertificateFile|SSLCACertificatePath|SSLCARevocationFile|SSLCARevocationPath|SSLCertificateChainFile|SSLCertificateFile|SSLCertificateKeyFile|SSLCipherSuite|SSLEngine|SSLMutex|SSLOptions|SSLPassPhraseDialog|SSLProtocol|SSLProxyCACertificateFile|SSLProxyCACertificatePath|SSLProxyCARevocationFile|SSLProxyCARevocationPath|SSLProxyCipherSuite|SSLProxyEngine|SSLProxyMachineCertificateFile|SSLProxyMachineCertificatePath|SSLProxyProtocol|SSLProxyVerify|SSLProxyVerifyDepth|SSLRandomSeed|SSLRequire|SSLRequireSSL|SSLSessionCache|SSLSessionCacheTimeout|SSLUserName|SSLVerifyClient|SSLVerifyDepth|SSLInsecureRenegotiation|SSLOpenSSLConfCmd)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642443489902592,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Substitute|SubstituteInheritBefore|SubstituteMaxLineLength)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642447784869888,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(CookieDomain|CookieExpires|CookieName|CookieStyle|CookieTracking)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642452079837184,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(VirtualDocumentRoot|VirtualDocumentRootIP|VirtualScriptAlias|VirtualScriptAliasIP)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52642456374804480,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(php_value|php_flag|php_admin_value|php_admin_flag)\\b(\\s+(.+?)(\\s+(\".+?\"|.+?))?)?\\s"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636069758435328,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392220818767872,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 55452146670436352,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(%\\{)((HTTP_USER_AGENT|HTTP_REFERER|HTTP_COOKIE|HTTP_FORWARDED|HTTP_HOST|HTTP_PROXY_CONNECTION|HTTP_ACCEPT|REMOTE_ADDR|REMOTE_HOST|REMOTE_PORT|REMOTE_USER|REMOTE_IDENT|REQUEST_METHOD|SCRIPT_FILENAME|PATH_INFO|QUERY_STRING|AUTH_TYPE|DOCUMENT_ROOT|SERVER_ADMIN|SERVER_NAME|SERVER_ADDR|SERVER_PORT|SERVER_PROTOCOL|SERVER_SOFTWARE|TIME_YEAR|TIME_MON|TIME_DAY|TIME_HOUR|TIME_MIN|TIME_SEC|TIME_WDAY|TIME|API_VERSION|THE_REQUEST|REQUEST_URI|REQUEST_FILENAME|IS_SUBREQ|HTTPS)|(.*?))(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288547712106496,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258541836599296,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49263764516831232,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288547712106496,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((text|image|application|video|audio)/.+?)\\s"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59397860110827520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(export|from|unset|set|on|off)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59396919512989696,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(\\d+)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 23362423066984448,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s(\\[)(.*?)(\\])\\s"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629336604755,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55452245454684160,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629336604755,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }