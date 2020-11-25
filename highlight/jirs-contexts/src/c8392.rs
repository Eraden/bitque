
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
        a: 844966096011264,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844966096011264,
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
      regex: Regex::new("\\#.*"),
      scope: vec![
        Scope {
            a: 51510711032872960,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(events) +\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500211838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8341 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(http) +\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500211838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8342 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(types) +\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500211838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8353 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(server) +\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500211838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8364 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(location) +[\\^]?~[\\*]? +(.*?)\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500213064,
            b: 35465847065542656,
        },
    ]),(2, vec![
        Scope {
            a: 55450759398817792,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8375 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(location) +(.*?)\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500213064,
            b: 35465847065542656,
        },
    ]),(2, vec![
        Scope {
            a: 59392130656699720,
            b: 35465847065542656,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8383 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(upstream) +(.*?)\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500211838,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130656699720,
            b: 35465847065542656,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8384 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(if) +\\("),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636696936448,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8385 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8386 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(daemon|env|debug_points|error_log|include|lock_file|master_process|pid|ssl_engine|timer_resolution|user|worker_cpu_affinity|worker_priority|worker_processes|worker_rlimit_core|worker_rlimit_nofile|worker_rlimit_sigpending|working_directory)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414813396992,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8387 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(accept_mutex|accept_mutex_delay|debug_connection|devpoll_changes|devpoll_events|epoll_events|kqueue_changes|kqueue_events|multi_accept|rtsig_signo|rtsig_overflow_events|rtsig_overflow_test|rtsig_overflow_threshold|use|worker_connections)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855145811,
            b: 35465847065542656,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8343 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(aio|alias|chunked_transfer_encoding|client_body_in_file_only|client_body_in_single_buffer|client_body_buffer_size|client_body_temp_path|client_body_timeout|client_header_buffer_size|client_header_timeout|client_max_body_size|connection_pool_size|default_type|directio|error_page|if_modified_since|internal|keepalive_disable|keepalive_timeout|keepalive_requests|large_client_header_buffers|limit_except|limit_rate|limit_rate_after|lingering_close|lingering_time|lingering_timeout|listen|log_not_found|log_subrequest|msie_padding|msie_refresh|open_file_cache|open_file_cache_errors|open_file_cache_min_uses|open_file_cache_valid|optimize_server_names|port_in_redirect|post_action|recursive_error_pages|request_pool_size|reset_timedout_connection|resolver|resolver_timeout|root|satisfy|satisfy_any|send_timeout|sendfile|server_name|server_name_in_redirect|server_names_hash_max_size|server_names_hash_bucket_size|server_tokens|tcp_nodelay|tcp_nopush|try_files|types\\ |underscores_in_hashes|variables_hash_bucket_size|variables_hash_max_size|types_hash_max_size)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 35465847065542656,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8344 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(set_real_ip_from|real_ip_recursive|real_ip_header)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 673007210481057792,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(allow|deny)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 109212832129613824,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8346 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(auth_basic|auth_basic_user_file)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 673288685457768448,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8347 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(autoindex|autoindex_exact_size|autoindex_localtime)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 427279555812655104,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8348 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ancient_browser|ancient_browser_value|modern_browser|modern_browser_value)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 673570160434479104,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8349 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(charset|charset_map|override_charset|source_charset)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 136515904870547456,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8350 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(empty_gif)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 673851635411189760,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8351 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(fastcgi_bind|fastcgi_buffer_size|fastcgi_buffers|fastcgi_cache|fastcgi_cache_key|fastcgi_cache_path|fastcgi_cache_methods|fastcgi_cache_min_uses|fastcgi_cache_use_stale|fastcgi_cache_valid|fastcgi_connect_timeout|fastcgi_index|fastcgi_hide_header|fastcgi_ignore_client_abort|fastcgi_ignore_headers|fastcgi_intercept_errors|fastcgi_max_temp_file_size|fastcgi_no_cache|fastcgi_next_upstream|fastcgi_param|fastcgi_pass|fastcgi_pass_header|fastcgi_pass_request_body|fastcgi_pass_request_headers|fastcgi_read_timeout|fastcgi_redirect_errors|fastcgi_send_timeout|fastcgi_split_path_info|fastcgi_store|fastcgi_store_access|fastcgi_temp_path)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 674133110387900416,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8352 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(geo)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 674414585364611072,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8354 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(gzip|gzip_buffers|gzip_comp_level|gzip_disable|gzip_http_version|gzip_min_length|gzip_proxied|gzip_types|gzip_vary)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 674696060341321728,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8355 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(gzip_static|gzip_disable|gzip_http_version|gzip_proxied|gzip_vary)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 674977535318032384,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8356 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(add_header|expires)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 430938730509893632,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8357 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(index)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 163818977611481088,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8358 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(limit_req_log_level|limit_req_zone|limit_req)\\b"),
      scope: vec![
        Scope {
            a: 47288629322514432,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 675259010294743040,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(limit_zone|limit_conn|limit_conn_log_level)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 675540485271453696,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8359 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(limit_conn_zone|limit_conn|limit_conn_log_level)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 675821960248164352,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8360 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(access_log|log_format|open_log_file_cache)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 7881840513777664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8361 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(map) +(\\$[A-Za-z0-9\\_]+) +(\\$[A-Za-z0-9\\_]+) *\\{"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576500211838,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087300263936,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087300263936,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8362 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(map_hash_max_size|map_hash_bucket_size)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 199847774630445056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8363 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(memcached_pass|memcached_connect_timeout|memcached_read_timeout|memcached_send_timeout|memcached_buffer_size|memcached_next_upstream)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 676103435224875008,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8365 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(proxy_bind|proxy_buffer_size|proxy_buffering|proxy_buffers|proxy_busy_buffers_size|proxy_cache|proxy_cache_bypass|proxy_cache_key|proxy_cache_methods|proxy_cache_min_uses|proxy_cache_path|proxy_cache_use_stale|proxy_cache_valid|proxy_connect_timeout|proxy_headers_hash_bucket_size|proxy_headers_hash_max_size|proxy_hide_header|proxy_ignore_client_abort|proxy_ignore_headers|proxy_intercept_errors|proxy_max_temp_file_size|proxy_method|proxy_next_upstream|proxy_no_cache|proxy_pass|proxy_http_version|proxy_pass_header|proxy_pass_request_body|proxy_pass_request_headers|proxy_redirect|proxy_read_timeout|proxy_redirect_errors|proxy_send_lowat|proxy_send_timeout|proxy_set_body|proxy_set_header|proxy_ssl_session_reuse|proxy_store|proxy_store_access|proxy_temp_file_write_size|proxy_temp_path|proxy_upstream_fail_timeout|proxy_upstream_max_fails)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 432909055346868224,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8366 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(valid_referers)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 676384910201585664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8367 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(rewrite_log|set|uninitialized_variable_warn)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 433190530323578880,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8368 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(break|return)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255135560853,
            b: 433190530323578880,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8369 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(rewrite)\\b *(\".+?(?<!\\\\)\"|\'.+?(?<!\\\\)\'|((.+?)(?<!\\\\)\\s))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 433190530323578880,
        },
    ]),(2, vec![
        Scope {
            a: 55450759398817792,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8370 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(scgi_bind|scgi_buffer_size|scgi_buffering|scgi_buffers|scgi_busy_buffers_size|scgi_cache|scgi_cache_bypass|scgi_cache_key|scgi_cache_methods|scgi_cache_min_uses|scgi_cache_path|scgi_cache_use_stale|scgi_cache_valid|scgi_connect_timeout|scgi_hide_header|scgi_ignore_client_abort|scgi_ignore_headers|scgi_intercept_errors|scgi_max_temp_file_size|scgi_next_upstream|scgi_no_cache|scgi_param|scgi_pass|scgi_pass_header|scgi_pass_request_body|scgi_pass_request_headers|scgi_read_timeout|scgi_send_timeout|scgi_store|scgi_store_access|scgi_temp_file_write_size|scgi_temp_path)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 676666385178296320,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8371 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(split_clients)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 676947860155006976,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8372 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ssi|ssi_silent_errors|ssi_types|ssi_value_length)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 677229335131717632,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8373 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ssl|ssl_certificate|ssl_certificate_key|ssl_client_certificate|ssl_dhparam|ssl_ciphers|ssl_crl|ssl_prefer_server_ciphers|ssl_protocols|ssl_verify_client|ssl_verify_depth|ssl_session_cache|ssl_session_timeout|ssl_engine)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 434034955253710848,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8374 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ip_hash|server)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 672725735504347136,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8376 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(userid|userid_domain|userid_expires|userid_name|userid_p3p|userid_path|userid_service)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 677510810108428288,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8377 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(uwsgi_bind|uwsgi_buffer_size|uwsgi_buffering|uwsgi_buffers|uwsgi_busy_buffers_size|uwsgi_cache|uwsgi_cache_bypass|uwsgi_cache_key|uwsgi_cache_methods|uwsgi_cache_min_uses|uwsgi_cache_path|uwsgi_cache_use_stale|uwsgi_cache_valid|uwsgi_connect_timeout|uwsgi_hide_header|uwsgi_ignore_client_abort|uwsgi_ignore_headers|uwsgi_intercept_errors|uwsgi_max_temp_file_size|uwsgi_modifier1|uwsgi_modifier2|uwsgi_next_upstream|uwsgi_no_cache|uwsgi_param|uwsgi_pass|uwsgi_pass_header|uwsgi_pass_request_body|uwsgi_pass_request_headers|uwsgi_read_timeout|uwsgi_send_timeout|uwsgi_store|uwsgi_store_access|uwsgi_string|uwsgi_temp_file_write_size|uwsgi_temp_path)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 677792285085138944,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8378 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b.*_by_lua_block\\s+\\{"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3106 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 8379 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([a-zA-Z0-9\\_]+)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855145833,
            b: 35465847065542656,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8380 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(stub_status)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 678355235038560256,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8381 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([a-z]+\\/[a-z0-9\\-\\.\\+]+)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638414855144597,
            b: 35465847065542656,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8382 }),
    ]),
      with_prototype: None
    }),
]
} }