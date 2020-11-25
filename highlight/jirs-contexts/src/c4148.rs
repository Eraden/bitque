
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
      regex: Regex::new("\\b(?xi:\n  apc_add | apc_bin_dump | apc_bin_dumpfile | apc_bin_load | apc_bin_loadfile | apc_cache_info | apc_cas | apc_clear_cache |\n  apc_compile_file | apc_dec | apc_define_constants | apc_delete | apc_delete_file | apc_exists | apc_fetch | apc_inc |\n  apc_load_constants | apc_sma_info | apc_store\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255160856634,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  array | array_change_key_case | array_chunk | array_column | array_combine | array_count_values | array_diff | array_diff_assoc |\n  array_diff_key | array_diff_uassoc | array_diff_ukey | array_fill | array_fill_keys | array_filter | array_flip | array_intersect |\n  array_intersect_assoc | array_intersect_key | array_intersect_uassoc | array_intersect_ukey | array_key_exists | array_keys | array_map | array_merge |\n  array_merge_recursive | array_multisort | array_pad | array_pop | array_product | array_push | array_rand | array_reduce |\n  array_replace | array_replace_recursive | array_reverse | array_search | array_shift | array_slice | array_splice | array_sum |\n  array_udiff | array_udiff_assoc | array_udiff_uassoc | array_uintersect | array_uintersect_assoc | array_uintersect_uassoc | array_unique | array_unshift |\n  array_values | array_walk | array_walk_recursive | arsort | asort | compact | count | current |\n  each | end | extract | in_array | key | krsort | ksort | list |\n  natcasesort | natsort | next | pos | prev | range | reset | rsort |\n  shuffle | sizeof | sort | uasort | uksort | usort\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255097417786,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  __halt_compiler | connection_aborted | connection_status | connection_timeout | constant | define | defined | die |\n  eval | exit | get_browser | highlight_file | highlight_string | ignore_user_abort | pack | php_check_syntax |\n  php_strip_whitespace | show_source | sleep | sys_getloadavg | time_nanosleep | time_sleep_until | uniqid | unpack |\n  usleep\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255160922170,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  bcadd | bccomp | bcdiv | bcmod | bcmul | bcpow | bcpowmod | bcscale |\n  bcsqrt | bcsub\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255160987706,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  bzclose | bzcompress | bzdecompress | bzerrno | bzerror | bzerrstr | bzflush | bzopen |\n  bzread | bzwrite\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161053242,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  FrenchToJD | GregorianToJD | JDDayOfWeek | JDMonthName | JDToFrench | JDToGregorian | JDToJulian | JewishToJD |\n  JulianToJD | cal_days_in_month | cal_from_jd | cal_info | cal_to_jd | easter_date | easter_days | jdtojewish |\n  jdtounix | unixtojd\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161118778,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  call_user_method | call_user_method_array | class_alias | class_exists | get_called_class | get_class | get_class_methods | get_class_vars |\n  get_declared_classes | get_declared_interfaces | get_object_vars | get_parent_class | interface_exists | is_a | is_subclass_of | method_exists |\n  property_exists\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161184314,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  com_addref | com_create_guid | com_event_sink | com_get | com_get_active_object | com_invoke | com_isenum | com_load |\n  com_load_typelib | com_message_pump | com_print_typeinfo | com_propget | com_propput | com_propset | com_release | com_set |\n  variant_abs | variant_add | variant_and | variant_cast | variant_cat | variant_cmp | variant_date_from_timestamp | variant_date_to_timestamp |\n  variant_div | variant_eqv | variant_fix | variant_get_type | variant_idiv | variant_imp | variant_int | variant_mod |\n  variant_mul | variant_neg | variant_not | variant_or | variant_pow | variant_round | variant_set | variant_set_type |\n  variant_sub | variant_xor\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161249850,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  random_bytes | random_int\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161315386,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ctype_alnum | ctype_alpha | ctype_cntrl | ctype_digit | ctype_graph | ctype_lower | ctype_print | ctype_punct |\n  ctype_space | ctype_upper | ctype_xdigit\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161380922,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  curl_close | curl_copy_handle | curl_errno | curl_error | curl_exec | curl_getinfo | curl_init | curl_multi_add_handle |\n  curl_multi_close | curl_multi_exec | curl_multi_getcontent | curl_multi_info_read | curl_multi_init | curl_multi_remove_handle | curl_multi_select | curl_setopt |\n  curl_setopt_array | curl_version\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161446458,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  checkdate | date | date_add | date_create | date_create_from_format | date_date_set | date_default_timezone_get | date_default_timezone_set |\n  date_diff | date_format | date_get_last_errors | date_interval_create_from_date_string | date_interval_format | date_isodate_set | date_modify | date_offset_get |\n  date_parse | date_parse_from_format | date_sub | date_sun_info | date_sunrise | date_sunset | date_time_set | date_timestamp_get |\n  date_timestamp_set | date_timezone_get | date_timezone_set | getdate | gettimeofday | gmdate | gmmktime | gmstrftime |\n  hrtime | idate | localtime | microtime | mktime | strftime | strptime | strtotime |\n  time | timezone_abbreviations_list | timezone_identifiers_list | timezone_location_get | timezone_name_from_abbr | timezone_name_get | timezone_offset_get | timezone_open |\n  timezone_transitions_get | timezone_version_get\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161511994,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  dba_close | dba_delete | dba_exists | dba_fetch | dba_firstkey | dba_handlers | dba_insert | dba_key_split |\n  dba_list | dba_nextkey | dba_open | dba_optimize | dba_popen | dba_replace | dba_sync\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161577530,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  dbx_close | dbx_compare | dbx_connect | dbx_error | dbx_escape_string | dbx_fetch_row | dbx_query | dbx_sort\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161643066,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  chdir | chroot | closedir | dir | getcwd | opendir | readdir | rewinddir |\n  scandir\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161708602,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  domxml_new_doc | domxml_open_file | domxml_open_mem | domxml_version | domxml_xmltree | domxml_xslt_stylesheet | domxml_xslt_stylesheet_doc | domxml_xslt_stylesheet_file |\n  domxml_xslt_version | xpath_eval | xpath_eval_expression | xpath_new_context | xpath_register_ns | xpath_register_ns_auto | xptr_eval | xptr_new_context\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161774138,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  dotnet_load\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161839674,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  enchant_broker_describe | enchant_broker_dict_exists | enchant_broker_free | enchant_broker_free_dict | enchant_broker_get_error | enchant_broker_init | enchant_broker_list_dicts | enchant_broker_request_dict |\n  enchant_broker_request_pwl_dict | enchant_broker_set_ordering | enchant_dict_add_to_personal | enchant_dict_add_to_session | enchant_dict_check | enchant_dict_describe | enchant_dict_get_error | enchant_dict_is_in_session |\n  enchant_dict_quick_check | enchant_dict_store_replacement | enchant_dict_suggest\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161905210,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ereg | ereg_replace | eregi | eregi_replace | split | spliti | sql_regcase\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255161970746,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  debug_backtrace | debug_print_backtrace | error_clear_last | error_get_last | error_log | error_reporting | restore_error_handler | restore_exception_handler |\n  set_error_handler | set_exception_handler | trigger_error | user_error\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162036282,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  escapeshellarg | escapeshellcmd | exec | passthru | proc_close | proc_get_status | proc_nice | proc_open |\n  proc_terminate | shell_exec | system\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255131037754,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  exif_imagetype | exif_read_data | exif_tagname | exif_thumbnail | read_exif_data\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162101818,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  basename | chgrp | chmod | chown | clearstatcache | copy | delete | dirname |\n  disk_free_space | disk_total_space | diskfreespace | fclose | feof | fflush | fgetc | fgetcsv |\n  fgets | fgetss | file | file_exists | file_get_contents | file_put_contents | fileatime | filectime |\n  filegroup | fileinode | filemtime | fileowner | fileperms | filesize | filetype | flock |\n  fnmatch | fopen | fpassthru | fputcsv | fputs | fread | fscanf | fseek |\n  fstat | ftell | ftruncate | fwrite | glob | is_dir | is_executable | is_file |\n  is_link | is_readable | is_uploaded_file | is_writable | is_writeable | lchgrp | lchown | link |\n  linkinfo | lstat | mkdir | move_uploaded_file | parse_ini_file | parse_ini_string | pathinfo | pclose |\n  popen | readfile | readlink | realpath | realpath_cache_get | realpath_cache_size | rename | rewind |\n  rmdir | set_file_buffer | stat | symlink | tempnam | tmpfile | touch | umask |\n  unlink\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255107117114,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  finfo_buffer | finfo_close | finfo_file | finfo_open | finfo_set_flags | mime_content_type\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162167354,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  filter_has_var | filter_id | filter_input | filter_input_array | filter_list | filter_var | filter_var_array\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255119700026,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  call_user_func | call_user_func_array | create_function | forward_static_call | forward_static_call_array | func_get_arg | func_get_args | func_num_args |\n  function_exists | get_defined_functions | register_shutdown_function | register_tick_function | unregister_tick_function\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162232890,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  bind_textdomain_codeset | bindtextdomain | dcgettext | dcngettext | dgettext | dngettext | gettext | ngettext |\n  textdomain\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162298426,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  gmp_abs | gmp_add | gmp_and | gmp_binomial | gmp_clrbit | gmp_cmp | gmp_com | gmp_div |\n  gmp_fact | gmp_gcd | gmp_gcdext | gmp_hamdist | gmp_init | gmp_intval | gmp_invert | gmp_jacobi |\n  gmp_kronecker | gmp_kronecker_q | gmp_kronecker_qr | gmp_kronecker_r | gmp_kroneckerexact | gmp_lcm | gmp_legendre | gmp_mod |\n  gmp_mul | gmp_neg | gmp_nextprime | gmp_or | gmp_perfect_power | gmp_perfect_square | gmp_popcount | gmp_pow |\n  gmp_powm | gmp_prob_prime | gmp_random | gmp_scan0 | gmp_scan1 | gmp_setbit | gmp_sign | gmp_sqrt |\n  gmp_sqrtrem | gmp_strval | gmp_sub | gmp_testbit | gmp_xor\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162363962,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  hash | hash_algos | hash_copy | hash_file | hash_final | hash_hkdf | hash_hmac | hash_hmac_file |\n  hash_init | hash_update | hash_update_file | hash_update_stream\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255111180346,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  http_build_cookie | http_build_str | http_build_url | http_cache_etag | http_cache_last_modified | http_chunked_decode | http_date | http_deflate |\n  http_get | http_get_request_body | http_get_request_body_stream | http_get_request_headers | http_head | http_inflate | http_match_etag | http_match_modified |\n  http_match_request_header | http_negotiate_charset | http_negotiate_content_type | http_negotiate_language | http_parse_cookie | http_parse_headers | http_parse_message | http_parse_params |\n  http_persistent_handles_clean | http_persistent_handles_count | http_persistent_handles_ident | http_post_data | http_post_fields | http_put_data | http_put_file | http_put_stream |\n  http_redirect | http_request | http_request_body_encode | http_request_method_exists | http_request_method_name | http_request_method_register | http_request_method_unregister | http_response_code |\n  http_send_content_disposition | http_send_content_type | http_send_data | http_send_file | http_send_last_modified | http_send_status | http_send_stream | http_support |\n  http_throttle | ob_deflatehandler | ob_etaghandler | ob_inflatehandler\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162429498,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  iconv | iconv_get_encoding | iconv_mime_decode | iconv_mime_decode_headers | iconv_mime_encode | iconv_set_encoding | iconv_strlen | iconv_strpos |\n  iconv_strrpos | iconv_substr | ob_iconv_handler\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162495034,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  iis_add_server | iis_get_dir_security | iis_get_script_map | iis_get_server_by_comment | iis_get_server_by_path | iis_get_server_rights | iis_get_service_state | iis_remove_server |\n  iis_set_app_settings | iis_set_dir_security | iis_set_script_map | iis_set_server_rights | iis_start_server | iis_start_service | iis_stop_server | iis_stop_service\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162560570,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  gd_info | getimagesize | image2wbmp | image_type_to_extension | image_type_to_mime_type | imagealphablending | imageantialias | imagearc |\n  imagechar | imagecharup | imagecolorallocate | imagecolorallocatealpha | imagecolorat | imagecolorclosest | imagecolorclosestalpha | imagecolorclosesthwb |\n  imagecolordeallocate | imagecolorexact | imagecolorexactalpha | imagecolormatch | imagecolorresolve | imagecolorresolvealpha | imagecolorset | imagecolorsforindex |\n  imagecolorstotal | imagecolortransparent | imageconvolution | imagecopy | imagecopymerge | imagecopymergegray | imagecopyresampled | imagecopyresized |\n  imagecreate | imagecreatefromgd | imagecreatefromgd2 | imagecreatefromgd2part | imagecreatefromgif | imagecreatefromjpeg | imagecreatefrompng | imagecreatefromstring |\n  imagecreatefromwbmp | imagecreatefromxbm | imagecreatefromxpm | imagecreatetruecolor | imagedashedline | imagedestroy | imageellipse | imagefill |\n  imagefilledarc | imagefilledellipse | imagefilledpolygon | imagefilledrectangle | imagefilltoborder | imagefilter | imagefontheight | imagefontwidth |\n  imageftbbox | imagefttext | imagegammacorrect | imagegd | imagegd2 | imagegif | imagegrabscreen | imagegrabwindow |\n  imageinterlace | imageistruecolor | imagejpeg | imagelayereffect | imageline | imageloadfont | imagepalettecopy | imagepng |\n  imagepolygon | imagepsbbox | imagepsencodefont | imagepsextendfont | imagepsfreefont | imagepsloadfont | imagepsslantfont | imagepstext |\n  imagerectangle | imagerotate | imagesavealpha | imagesetbrush | imagesetpixel | imagesetstyle | imagesetthickness | imagesettile |\n  imagestring | imagestringup | imagesx | imagesy | imagetruecolortopalette | imagettfbbox | imagettftext | imagetypes |\n  imagewbmp | imagexbm | iptcembed | iptcparse | jpeg2wbmp | png2wbmp\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255120683066,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  assert | assert_options | dl | extension_loaded | gc_collect_cycles | gc_disable | gc_enable | gc_enabled |\n  get_cfg_var | get_current_user | get_defined_constants | get_extension_funcs | get_include_path | get_included_files | get_loaded_extensions | get_magic_quotes_gpc |\n  get_magic_quotes_runtime | get_required_files | getenv | getlastmod | getmygid | getmyinode | getmypid | getmyuid |\n  getopt | getrusage | ini_alter | ini_get | ini_get_all | ini_restore | ini_set | magic_quotes_runtime |\n  main | memory_get_peak_usage | memory_get_usage | php_ini_loaded_file | php_ini_scanned_files | php_logo_guid | php_sapi_name | php_uname |\n  phpcredits | phpinfo | phpversion | putenv | restore_include_path | set_include_path | set_magic_quotes_runtime | set_time_limit |\n  sys_get_temp_dir | version_compare | zend_logo_guid | zend_thread_id | zend_version\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162626106,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ibase_add_user | ibase_affected_rows | ibase_backup | ibase_blob_add | ibase_blob_cancel | ibase_blob_close | ibase_blob_create | ibase_blob_echo |\n  ibase_blob_get | ibase_blob_import | ibase_blob_info | ibase_blob_open | ibase_close | ibase_commit | ibase_commit_ret | ibase_connect |\n  ibase_db_info | ibase_delete_user | ibase_drop_db | ibase_errcode | ibase_errmsg | ibase_execute | ibase_fetch_assoc | ibase_fetch_object |\n  ibase_fetch_row | ibase_field_info | ibase_free_event_handler | ibase_free_query | ibase_free_result | ibase_gen_id | ibase_maintain_db | ibase_modify_user |\n  ibase_name_result | ibase_num_fields | ibase_num_params | ibase_param_info | ibase_pconnect | ibase_prepare | ibase_query | ibase_restore |\n  ibase_rollback | ibase_rollback_ret | ibase_server_info | ibase_service_attach | ibase_service_detach | ibase_set_event_handler | ibase_timefmt | ibase_trans |\n  ibase_wait_event\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162691642,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  collator_asort | collator_compare | collator_create | collator_get_attribute | collator_get_error_code | collator_get_error_message | collator_get_locale | collator_get_sort_key |\n  collator_get_strength | collator_set_attribute | collator_set_strength | collator_sort | collator_sort_with_sort_keys | datefmt_create | datefmt_format | datefmt_get_calendar |\n  datefmt_get_datetype | datefmt_get_error_code | datefmt_get_error_message | datefmt_get_locale | datefmt_get_pattern | datefmt_get_timetype | datefmt_get_timezone_id | datefmt_is_lenient |\n  datefmt_localtime | datefmt_parse | datefmt_set_calendar | datefmt_set_lenient | datefmt_set_pattern | datefmt_set_timezone_id | grapheme_extract | grapheme_stripos |\n  grapheme_stristr | grapheme_strlen | grapheme_strpos | grapheme_strripos | grapheme_strrpos | grapheme_strstr | grapheme_substr | idn_to_ascii |\n  idn_to_unicode | idn_to_utf8 | intl_error_name | intl_get_error_code | intl_get_error_message | intl_is_failure | locale_accept_from_http | locale_compose |\n  locale_filter_matches | locale_get_all_variants | locale_get_default | locale_get_display_language | locale_get_display_name | locale_get_display_region | locale_get_display_script | locale_get_display_variant |\n  locale_get_keywords | locale_get_primary_language | locale_get_region | locale_get_script | locale_lookup | locale_parse | locale_set_default | msgfmt_create |\n  msgfmt_format | msgfmt_format_message | msgfmt_get_error_code | msgfmt_get_error_message | msgfmt_get_locale | msgfmt_get_pattern | msgfmt_parse | msgfmt_parse_message |\n  msgfmt_set_pattern | normalizer_is_normalized | normalizer_normalize | numfmt_create | numfmt_format | numfmt_format_currency | numfmt_get_attribute | numfmt_get_error_code |\n  numfmt_get_error_message | numfmt_get_locale | numfmt_get_pattern | numfmt_get_symbol | numfmt_get_text_attribute | numfmt_parse | numfmt_parse_currency | numfmt_set_attribute |\n  numfmt_set_pattern | numfmt_set_symbol | numfmt_set_text_attribute | resourcebundle_count | resourcebundle_create | resourcebundle_get | resourcebundle_get_error_code | resourcebundle_get_error_message |\n  resourcebundle_locales | transliterator_create | transliterator_create_from_rules | transliterator_create_inverse | transliterator_get_error_code | transliterator_get_error_message | transliterator_list_ids | transliterator_transliterate\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162757178,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  json_decode | json_encode | json_last_error\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255088046138,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ldap_8859_to_t61 | ldap_add | ldap_bind | ldap_close | ldap_compare | ldap_connect | ldap_count_entries | ldap_delete |\n  ldap_dn2ufn | ldap_err2str | ldap_errno | ldap_error | ldap_explode_dn | ldap_first_attribute | ldap_first_entry | ldap_first_reference |\n  ldap_free_result | ldap_get_attributes | ldap_get_dn | ldap_get_entries | ldap_get_option | ldap_get_values | ldap_get_values_len | ldap_list |\n  ldap_mod_add | ldap_mod_del | ldap_mod_replace | ldap_modify | ldap_next_attribute | ldap_next_entry | ldap_next_reference | ldap_parse_reference |\n  ldap_parse_result | ldap_read | ldap_rename | ldap_sasl_bind | ldap_search | ldap_set_option | ldap_set_rebind_proc | ldap_sort |\n  ldap_start_tls | ldap_t61_to_8859 | ldap_unbind\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162822714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  libxml_clear_errors | libxml_disable_entity_loader | libxml_get_errors | libxml_get_last_error | libxml_set_streams_context | libxml_use_internal_errors\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162888250,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ezmlm_hash | mail\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255162953786,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  abs | acos | acosh | asin | asinh | atan | atan2 | atanh |\n  base_convert | bindec | ceil | cos | cosh | decbin | dechex | decoct |\n  deg2rad | exp | expm1 | floor | fmod | getrandmax | hexdec | hypot |\n  intdiv | is_finite | is_infinite | is_nan | lcg_value | log | log10 | log1p |\n  max | min | mt_getrandmax | mt_rand | mt_srand | octdec | pi | pow |\n  rad2deg | rand | round | sin | sinh | sqrt | srand | tan |\n  tanh\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255142441018,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mb_check_encoding | mb_convert_case | mb_convert_encoding | mb_convert_kana | mb_convert_variables | mb_decode_mimeheader | mb_decode_numericentity | mb_detect_encoding |\n  mb_detect_order | mb_encode_mimeheader | mb_encode_numericentity | mb_encoding_aliases | mb_ereg | mb_ereg_match | mb_ereg_replace | mb_ereg_search |\n  mb_ereg_search_getpos | mb_ereg_search_getregs | mb_ereg_search_init | mb_ereg_search_pos | mb_ereg_search_regs | mb_ereg_search_setpos | mb_eregi | mb_eregi_replace |\n  mb_get_info | mb_http_input | mb_http_output | mb_internal_encoding | mb_language | mb_list_encodings | mb_output_handler | mb_parse_str |\n  mb_preferred_mime_name | mb_regex_encoding | mb_regex_set_options | mb_send_mail | mb_split | mb_strcut | mb_strimwidth | mb_stripos |\n  mb_stristr | mb_strlen | mb_strpos | mb_strrchr | mb_strrichr | mb_strripos | mb_strrpos | mb_strstr |\n  mb_strtolower | mb_strtoupper | mb_strwidth | mb_substitute_character | mb_substr | mb_substr_count\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163019322,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mcrypt_cbc | mcrypt_cfb | mcrypt_create_iv | mcrypt_decrypt | mcrypt_ecb | mcrypt_enc_get_algorithms_name | mcrypt_enc_get_block_size | mcrypt_enc_get_iv_size |\n  mcrypt_enc_get_key_size | mcrypt_enc_get_modes_name | mcrypt_enc_get_supported_key_sizes | mcrypt_enc_is_block_algorithm | mcrypt_enc_is_block_algorithm_mode | mcrypt_enc_is_block_mode | mcrypt_enc_self_test | mcrypt_encrypt |\n  mcrypt_generic | mcrypt_generic_deinit | mcrypt_generic_end | mcrypt_generic_init | mcrypt_get_block_size | mcrypt_get_cipher_name | mcrypt_get_iv_size | mcrypt_get_key_size |\n  mcrypt_list_algorithms | mcrypt_list_modes | mcrypt_module_close | mcrypt_module_get_algo_block_size | mcrypt_module_get_algo_key_size | mcrypt_module_get_supported_key_sizes | mcrypt_module_is_block_algorithm | mcrypt_module_is_block_algorithm_mode |\n  mcrypt_module_is_block_mode | mcrypt_module_open | mcrypt_module_self_test | mcrypt_ofb | mdecrypt_generic\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163084858,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  memcache_debug\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163150394,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mhash | mhash_count | mhash_get_block_size | mhash_get_hash_name | mhash_keygen_s2k\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163215930,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  bson_decode | bson_encode\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163281466,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mysql_affected_rows | mysql_client_encoding | mysql_close | mysql_connect | mysql_create_db | mysql_data_seek | mysql_db_name | mysql_db_query |\n  mysql_drop_db | mysql_errno | mysql_error | mysql_escape_string | mysql_fetch_array | mysql_fetch_assoc | mysql_fetch_field | mysql_fetch_lengths |\n  mysql_fetch_object | mysql_fetch_row | mysql_field_flags | mysql_field_len | mysql_field_name | mysql_field_seek | mysql_field_table | mysql_field_type |\n  mysql_free_result | mysql_get_client_info | mysql_get_host_info | mysql_get_proto_info | mysql_get_server_info | mysql_info | mysql_insert_id | mysql_list_dbs |\n  mysql_list_fields | mysql_list_processes | mysql_list_tables | mysql_num_fields | mysql_num_rows | mysql_pconnect | mysql_ping | mysql_query |\n  mysql_real_escape_string | mysql_result | mysql_select_db | mysql_set_charset | mysql_stat | mysql_tablename | mysql_thread_id | mysql_unbuffered_query\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163347002,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mysqli_affected_rows | mysqli_autocommit | mysqli_bind_param | mysqli_bind_result | mysqli_change_user | mysqli_character_set_name | mysqli_client_encoding | mysqli_close |\n  mysqli_commit | mysqli_connect | mysqli_connect_errno | mysqli_connect_error | mysqli_data_seek | mysqli_debug | mysqli_disable_reads_from_master | mysqli_disable_rpl_parse |\n  mysqli_dump_debug_info | mysqli_embedded_server_end | mysqli_embedded_server_start | mysqli_enable_reads_from_master | mysqli_enable_rpl_parse | mysqli_errno | mysqli_error | mysqli_escape_string |\n  mysqli_execute | mysqli_fetch | mysqli_fetch_all | mysqli_fetch_array | mysqli_fetch_assoc | mysqli_fetch_field | mysqli_fetch_field_direct | mysqli_fetch_fields |\n  mysqli_fetch_lengths | mysqli_fetch_object | mysqli_fetch_row | mysqli_field_count | mysqli_field_seek | mysqli_field_tell | mysqli_free_result | mysqli_get_cache_stats |\n  mysqli_get_charset | mysqli_get_client_info | mysqli_get_client_stats | mysqli_get_client_version | mysqli_get_connection_stats | mysqli_get_host_info | mysqli_get_metadata | mysqli_get_proto_info |\n  mysqli_get_server_info | mysqli_get_server_version | mysqli_get_warnings | mysqli_info | mysqli_init | mysqli_insert_id | mysqli_kill | mysqli_master_query |\n  mysqli_more_results | mysqli_multi_query | mysqli_next_result | mysqli_num_fields | mysqli_num_rows | mysqli_options | mysqli_param_count | mysqli_ping |\n  mysqli_poll | mysqli_prepare | mysqli_query | mysqli_real_connect | mysqli_real_escape_string | mysqli_real_query | mysqli_reap_async_query | mysqli_report |\n  mysqli_rollback | mysqli_rpl_parse_enabled | mysqli_rpl_probe | mysqli_rpl_query_type | mysqli_select_db | mysqli_send_long_data | mysqli_send_query | mysqli_set_charset |\n  mysqli_set_local_infile_default | mysqli_set_local_infile_handler | mysqli_set_opt | mysqli_slave_query | mysqli_sqlstate | mysqli_ssl_set | mysqli_stat | mysqli_stmt_affected_rows |\n  mysqli_stmt_attr_get | mysqli_stmt_attr_set | mysqli_stmt_bind_param | mysqli_stmt_bind_result | mysqli_stmt_close | mysqli_stmt_data_seek | mysqli_stmt_errno | mysqli_stmt_error |\n  mysqli_stmt_execute | mysqli_stmt_fetch | mysqli_stmt_field_count | mysqli_stmt_free_result | mysqli_stmt_get_result | mysqli_stmt_get_warnings | mysqli_stmt_init | mysqli_stmt_insert_id |\n  mysqli_stmt_num_rows | mysqli_stmt_param_count | mysqli_stmt_prepare | mysqli_stmt_reset | mysqli_stmt_result_metadata | mysqli_stmt_send_long_data | mysqli_stmt_sqlstate | mysqli_stmt_store_result |\n  mysqli_store_result | mysqli_thread_id | mysqli_thread_safe | mysqli_use_result | mysqli_warning_count\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163412538,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mysqlnd_ms_get_stats | mysqlnd_ms_query_is_select | mysqlnd_ms_set_user_pick_server\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163478074,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mysqlnd_qc_change_handler | mysqlnd_qc_clear_cache | mysqlnd_qc_get_cache_info | mysqlnd_qc_get_core_stats | mysqlnd_qc_get_handler | mysqlnd_qc_get_query_trace_log | mysqlnd_qc_set_user_handlers\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163543610,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  checkdnsrr | closelog | define_syslog_variables | dns_check_record | dns_get_mx | dns_get_record | fsockopen | gethostbyaddr |\n  gethostbyname | gethostbynamel | gethostname | getmxrr | getprotobyname | getprotobynumber | getservbyname | getservbyport |\n  header | header_remove | headers_list | headers_sent | inet_ntop | inet_pton | ip2long | long2ip |\n  openlog | pfsockopen | setcookie | setrawcookie | socket_get_status | socket_set_blocking | socket_set_timeout | syslog\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163609146,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  nsapi_request_headers | nsapi_response_headers | nsapi_virtual\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163674682,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  aggregate | aggregate_info | aggregate_methods | aggregate_methods_by_list | aggregate_methods_by_regexp | aggregate_properties | aggregate_properties_by_list | aggregate_properties_by_regexp |\n  aggregation_info | deaggregate\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163740218,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  oci_bind_array_by_name | oci_bind_by_name | oci_cancel | oci_client_version | oci_close | oci_commit | oci_connect | oci_define_by_name |\n  oci_error | oci_execute | oci_fetch | oci_fetch_all | oci_fetch_array | oci_fetch_assoc | oci_fetch_object | oci_fetch_row |\n  oci_field_is_null | oci_field_name | oci_field_precision | oci_field_scale | oci_field_size | oci_field_type | oci_field_type_raw | oci_free_statement |\n  oci_internal_debug | oci_lob_copy | oci_lob_is_equal | oci_new_collection | oci_new_connect | oci_new_cursor | oci_new_descriptor | oci_num_fields |\n  oci_num_rows | oci_parse | oci_password_change | oci_pconnect | oci_result | oci_rollback | oci_server_version | oci_set_action |\n  oci_set_call_timeout | oci_set_client_identifier | oci_set_client_info | oci_set_db_operation | oci_set_edition | oci_set_module_name | oci_set_prefetch | oci_statement_type |\n  ocibindbyname | ocicancel | ocicloselob | ocicollappend | ocicollassign | ocicollassignelem | ocicollgetelem | ocicollmax |\n  ocicollsize | ocicolltrim | ocicolumnisnull | ocicolumnname | ocicolumnprecision | ocicolumnscale | ocicolumnsize | ocicolumntype |\n  ocicolumntyperaw | ocicommit | ocidefinebyname | ocierror | ociexecute | ocifetch | ocifetchinto | ocifetchstatement |\n  ocifreecollection | ocifreecursor | ocifreedesc | ocifreestatement | ociinternaldebug | ociloadlob | ocilogoff | ocilogon |\n  ocinewcollection | ocinewcursor | ocinewdescriptor | ocinlogon | ocinumcols | ociparse | ociplogon | ociresult |\n  ocirollback | ocirowcount | ocisavelob | ocisavelobfile | ociserverversion | ocisetprefetch | ocistatementtype | ociwritelobtofile |\n  ociwritetemporarylob\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163805754,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  openssl_cipher_iv_length | openssl_csr_export | openssl_csr_export_to_file | openssl_csr_get_public_key | openssl_csr_get_subject | openssl_csr_new | openssl_csr_sign | openssl_decrypt |\n  openssl_dh_compute_key | openssl_digest | openssl_encrypt | openssl_error_string | openssl_free_key | openssl_get_cipher_methods | openssl_get_md_methods | openssl_get_privatekey |\n  openssl_get_publickey | openssl_open | openssl_pkcs12_export | openssl_pkcs12_export_to_file | openssl_pkcs12_read | openssl_pkcs7_decrypt | openssl_pkcs7_encrypt | openssl_pkcs7_sign |\n  openssl_pkcs7_verify | openssl_pkey_derive | openssl_pkey_export | openssl_pkey_export_to_file | openssl_pkey_free | openssl_pkey_get_details | openssl_pkey_get_private | openssl_pkey_get_public |\n  openssl_pkey_new | openssl_private_decrypt | openssl_private_encrypt | openssl_public_decrypt | openssl_public_encrypt | openssl_random_pseudo_bytes | openssl_seal | openssl_sign |\n  openssl_verify | openssl_x509_check_private_key | openssl_x509_checkpurpose | openssl_x509_export | openssl_x509_export_to_file | openssl_x509_free | openssl_x509_parse | openssl_x509_read\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163871290,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  flush | ob_clean | ob_end_clean | ob_end_flush | ob_flush | ob_get_clean | ob_get_contents | ob_get_flush |\n  ob_get_length | ob_get_level | ob_get_status | ob_gzhandler | ob_implicit_flush | ob_list_handlers | ob_start | output_add_rewrite_var |\n  output_reset_rewrite_vars\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255149322298,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  overload\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255163936826,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  pcntl_alarm | pcntl_exec | pcntl_fork | pcntl_getpriority | pcntl_setpriority | pcntl_signal | pcntl_signal_dispatch | pcntl_sigprocmask |\n  pcntl_sigtimedwait | pcntl_sigwaitinfo | pcntl_wait | pcntl_waitpid | pcntl_wexitstatus | pcntl_wifexited | pcntl_wifsignaled | pcntl_wifstopped |\n  pcntl_wstopsig | pcntl_wtermsig\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164002362,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  pg_affected_rows | pg_cancel_query | pg_client_encoding | pg_close | pg_connect | pg_connection_busy | pg_connection_reset | pg_connection_status |\n  pg_convert | pg_copy_from | pg_copy_to | pg_dbname | pg_delete | pg_end_copy | pg_escape_bytea | pg_escape_string |\n  pg_execute | pg_fetch_all | pg_fetch_all_columns | pg_fetch_array | pg_fetch_assoc | pg_fetch_object | pg_fetch_result | pg_fetch_row |\n  pg_field_is_null | pg_field_name | pg_field_num | pg_field_prtlen | pg_field_size | pg_field_table | pg_field_type | pg_field_type_oid |\n  pg_free_result | pg_get_notify | pg_get_pid | pg_get_result | pg_host | pg_insert | pg_last_error | pg_last_notice |\n  pg_last_oid | pg_lo_close | pg_lo_create | pg_lo_export | pg_lo_import | pg_lo_open | pg_lo_read | pg_lo_read_all |\n  pg_lo_seek | pg_lo_tell | pg_lo_unlink | pg_lo_write | pg_meta_data | pg_num_fields | pg_num_rows | pg_options |\n  pg_parameter_status | pg_pconnect | pg_ping | pg_port | pg_prepare | pg_put_line | pg_query | pg_query_params |\n  pg_result_error | pg_result_error_field | pg_result_seek | pg_result_status | pg_select | pg_send_execute | pg_send_prepare | pg_send_query |\n  pg_send_query_params | pg_set_client_encoding | pg_set_error_verbosity | pg_trace | pg_transaction_status | pg_tty | pg_unescape_bytea | pg_untrace |\n  pg_update | pg_version\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164067898,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  apache_child_terminate | apache_get_modules | apache_get_version | apache_getenv | apache_lookup_uri | apache_note | apache_request_headers | apache_reset_timeout |\n  apache_response_headers | apache_setenv | getallheaders | virtual\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164133434,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  dom_import_simplexml\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164198970,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ftp_alloc | ftp_cdup | ftp_chdir | ftp_chmod | ftp_close | ftp_connect | ftp_delete | ftp_exec |\n  ftp_fget | ftp_fput | ftp_get | ftp_get_option | ftp_login | ftp_mdtm | ftp_mkdir | ftp_nb_continue |\n  ftp_nb_fget | ftp_nb_fput | ftp_nb_get | ftp_nb_put | ftp_nlist | ftp_pasv | ftp_put | ftp_pwd |\n  ftp_quit | ftp_raw | ftp_rawlist | ftp_rename | ftp_rmdir | ftp_set_option | ftp_site | ftp_size |\n  ftp_ssl_connect | ftp_systype\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164264506,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  imap_8bit | imap_alerts | imap_append | imap_base64 | imap_binary | imap_body | imap_bodystruct | imap_check |\n  imap_clearflag_full | imap_close | imap_create | imap_createmailbox | imap_delete | imap_deletemailbox | imap_errors | imap_expunge |\n  imap_fetch_overview | imap_fetchbody | imap_fetchheader | imap_fetchmime | imap_fetchstructure | imap_fetchtext | imap_gc | imap_get_quota |\n  imap_get_quotaroot | imap_getacl | imap_getmailboxes | imap_getsubscribed | imap_header | imap_headerinfo | imap_headers | imap_last_error |\n  imap_list | imap_listmailbox | imap_listscan | imap_listsubscribed | imap_lsub | imap_mail | imap_mail_compose | imap_mail_copy |\n  imap_mail_move | imap_mailboxmsginfo | imap_mime_header_decode | imap_msgno | imap_num_msg | imap_num_recent | imap_open | imap_ping |\n  imap_qprint | imap_rename | imap_renamemailbox | imap_reopen | imap_rfc822_parse_adrlist | imap_rfc822_parse_headers | imap_rfc822_write_address | imap_savebody |\n  imap_scan | imap_scanmailbox | imap_search | imap_set_quota | imap_setacl | imap_setflag_full | imap_sort | imap_status |\n  imap_subscribe | imap_thread | imap_timeout | imap_uid | imap_undelete | imap_unsubscribe | imap_utf7_decode | imap_utf7_encode |\n  imap_utf8\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164330042,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  mssql_bind | mssql_close | mssql_connect | mssql_data_seek | mssql_execute | mssql_fetch_array | mssql_fetch_assoc | mssql_fetch_batch |\n  mssql_fetch_field | mssql_fetch_object | mssql_fetch_row | mssql_field_length | mssql_field_name | mssql_field_seek | mssql_field_type | mssql_free_result |\n  mssql_free_statement | mssql_get_last_message | mssql_guid_string | mssql_init | mssql_min_error_severity | mssql_min_message_severity | mssql_next_result | mssql_num_fields |\n  mssql_num_rows | mssql_pconnect | mssql_query | mssql_result | mssql_rows_affected | mssql_select_db\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164395578,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  odbc_autocommit | odbc_binmode | odbc_close | odbc_close_all | odbc_columnprivileges | odbc_columns | odbc_commit | odbc_connect |\n  odbc_cursor | odbc_data_source | odbc_do | odbc_error | odbc_errormsg | odbc_exec | odbc_execute | odbc_fetch_array |\n  odbc_fetch_into | odbc_fetch_object | odbc_fetch_row | odbc_field_len | odbc_field_name | odbc_field_num | odbc_field_precision | odbc_field_scale |\n  odbc_field_type | odbc_foreignkeys | odbc_free_result | odbc_gettypeinfo | odbc_longreadlen | odbc_next_result | odbc_num_fields | odbc_num_rows |\n  odbc_pconnect | odbc_prepare | odbc_primarykeys | odbc_procedurecolumns | odbc_procedures | odbc_result | odbc_result_all | odbc_rollback |\n  odbc_setoption | odbc_specialcolumns | odbc_statistics | odbc_tableprivileges | odbc_tables\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164461114,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  preg_filter | preg_grep | preg_last_error | preg_match | preg_match_all | preg_quote | preg_replace | preg_replace_callback |\n  preg_replace_callback_array | preg_split\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164526650,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  class_implements | class_parents | iterator_apply | iterator_count | iterator_to_array | spl_autoload | spl_autoload_call | spl_autoload_extensions |\n  spl_autoload_functions | spl_autoload_register | spl_autoload_unregister | spl_classes | spl_object_hash | spl_object_id\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164592186,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  zip_close | zip_entry_close | zip_entry_compressedsize | zip_entry_compressionmethod | zip_entry_filesize | zip_entry_name | zip_entry_open | zip_entry_read |\n  zip_open | zip_read\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164657722,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  posix_access | posix_ctermid | posix_errno | posix_get_last_error | posix_getcwd | posix_getegid | posix_geteuid | posix_getgid |\n  posix_getgrgid | posix_getgrnam | posix_getgroups | posix_getlogin | posix_getpgid | posix_getpgrp | posix_getpid | posix_getppid |\n  posix_getpwnam | posix_getpwuid | posix_getrlimit | posix_getsid | posix_getuid | posix_initgroups | posix_isatty | posix_kill |\n  posix_mkfifo | posix_mknod | posix_setegid | posix_seteuid | posix_setgid | posix_setpgid | posix_setsid | posix_setuid |\n  posix_strerror | posix_times | posix_ttyname | posix_uname\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164723258,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  setproctitle | setthreadtitle\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164788794,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  pspell_add_to_personal | pspell_add_to_session | pspell_check | pspell_clear_session | pspell_config_create | pspell_config_data_dir | pspell_config_dict_dir | pspell_config_ignore |\n  pspell_config_mode | pspell_config_personal | pspell_config_repl | pspell_config_runtogether | pspell_config_save_repl | pspell_new | pspell_new_config | pspell_new_personal |\n  pspell_save_wordlist | pspell_store_replacement | pspell_suggest\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164854330,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  readline | readline_add_history | readline_callback_handler_install | readline_callback_handler_remove | readline_callback_read_char | readline_clear_history | readline_completion_function | readline_info |\n  readline_list_history | readline_on_new_line | readline_read_history | readline_redisplay | readline_write_history\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164919866,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  recode | recode_file | recode_string\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255164985402,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  rrd_create | rrd_error | rrd_fetch | rrd_first | rrd_graph | rrd_info | rrd_last | rrd_lastupdate |\n  rrd_restore | rrd_tune | rrd_update | rrd_xport\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165050938,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ftok | msg_get_queue | msg_queue_exists | msg_receive | msg_remove_queue | msg_send | msg_set_queue | msg_stat_queue |\n  sem_acquire | sem_get | sem_release | sem_remove | shm_attach | shm_detach | shm_get_var | shm_has_var |\n  shm_put_var | shm_remove | shm_remove_var\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165116474,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  session_cache_expire | session_cache_limiter | session_commit | session_decode | session_destroy | session_encode | session_get_cookie_params | session_id |\n  session_is_registered | session_module_name | session_name | session_regenerate_id | session_register | session_save_path | session_set_cookie_params | session_set_save_handler |\n  session_start | session_unregister | session_unset | session_write_close\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165182010,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  shmop_close | shmop_delete | shmop_open | shmop_read | shmop_size | shmop_write\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165247546,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  simplexml_import_dom | simplexml_load_file | simplexml_load_string\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165313082,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  snmp2_get | snmp2_getnext | snmp2_real_walk | snmp2_set | snmp2_walk | snmp3_get | snmp3_getnext | snmp3_real_walk |\n  snmp3_set | snmp3_walk | snmp_get_quick_print | snmp_get_valueretrieval | snmp_read_mib | snmp_set_enum_print | snmp_set_oid_numeric_print | snmp_set_oid_output_format |\n  snmp_set_quick_print | snmp_set_valueretrieval | snmpget | snmpgetnext | snmprealwalk | snmpset | snmpwalk | snmpwalkoid\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165378618,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  is_soap_fault | use_soap_error_handler\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165444154,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  socket_accept | socket_bind | socket_clear_error | socket_close | socket_connect | socket_create | socket_create_listen | socket_create_pair |\n  socket_export_stream | socket_get_option | socket_getpeername | socket_getsockname | socket_last_error | socket_listen | socket_read | socket_recv |\n  socket_recvfrom | socket_select | socket_send | socket_sendto | socket_set_block | socket_set_nonblock | socket_set_option | socket_shutdown |\n  socket_strerror | socket_write\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165509690,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  sqlite_array_query | sqlite_busy_timeout | sqlite_changes | sqlite_close | sqlite_column | sqlite_create_aggregate | sqlite_create_function | sqlite_current |\n  sqlite_error_string | sqlite_escape_string | sqlite_exec | sqlite_factory | sqlite_fetch_all | sqlite_fetch_array | sqlite_fetch_column_types | sqlite_fetch_object |\n  sqlite_fetch_single | sqlite_fetch_string | sqlite_field_name | sqlite_has_more | sqlite_has_prev | sqlite_key | sqlite_last_error | sqlite_last_insert_rowid |\n  sqlite_libencoding | sqlite_libversion | sqlite_next | sqlite_num_fields | sqlite_num_rows | sqlite_open | sqlite_popen | sqlite_prev |\n  sqlite_query | sqlite_rewind | sqlite_seek | sqlite_single_query | sqlite_udf_decode_binary | sqlite_udf_encode_binary | sqlite_unbuffered_query | sqlite_valid\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165575226,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  stats_absolute_deviation | stats_cdf_beta | stats_cdf_binomial | stats_cdf_cauchy | stats_cdf_chisquare | stats_cdf_exponential | stats_cdf_f | stats_cdf_gamma |\n  stats_cdf_laplace | stats_cdf_logistic | stats_cdf_negative_binomial | stats_cdf_noncentral_chisquare | stats_cdf_noncentral_f | stats_cdf_poisson | stats_cdf_t | stats_cdf_uniform |\n  stats_cdf_weibull | stats_covariance | stats_den_uniform | stats_dens_beta | stats_dens_cauchy | stats_dens_chisquare | stats_dens_exponential | stats_dens_f |\n  stats_dens_gamma | stats_dens_laplace | stats_dens_logistic | stats_dens_negative_binomial | stats_dens_normal | stats_dens_pmf_binomial | stats_dens_pmf_hypergeometric | stats_dens_pmf_poisson |\n  stats_dens_t | stats_dens_weibull | stats_harmonic_mean | stats_kurtosis | stats_rand_gen_beta | stats_rand_gen_chisquare | stats_rand_gen_exponential | stats_rand_gen_f |\n  stats_rand_gen_funiform | stats_rand_gen_gamma | stats_rand_gen_ibinomial | stats_rand_gen_ibinomial_negative | stats_rand_gen_int | stats_rand_gen_ipoisson | stats_rand_gen_iuniform | stats_rand_gen_noncenral_chisquare |\n  stats_rand_gen_noncentral_f | stats_rand_gen_noncentral_t | stats_rand_gen_normal | stats_rand_gen_t | stats_rand_get_seeds | stats_rand_phrase_to_seeds | stats_rand_ranf | stats_rand_setall |\n  stats_skew | stats_standard_deviation | stats_stat_binomial_coef | stats_stat_correlation | stats_stat_gennch | stats_stat_independent_t | stats_stat_innerproduct | stats_stat_noncentral_t |\n  stats_stat_paired_t | stats_stat_percentile | stats_stat_powersum | stats_variance\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165640762,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  set_socket_blocking | stream_bucket_append | stream_bucket_make_writeable | stream_bucket_new | stream_bucket_prepend | stream_context_create | stream_context_get_default | stream_context_get_options |\n  stream_context_get_params | stream_context_set_default | stream_context_set_option | stream_context_set_params | stream_copy_to_stream | stream_encoding | stream_filter_append | stream_filter_prepend |\n  stream_filter_register | stream_filter_remove | stream_get_contents | stream_get_filters | stream_get_line | stream_get_meta_data | stream_get_transports | stream_get_wrappers |\n  stream_is_local | stream_notification_callback | stream_register_wrapper | stream_resolve_include_path | stream_select | stream_set_blocking | stream_set_read_buffer | stream_set_timeout |\n  stream_set_write_buffer | stream_socket_accept | stream_socket_client | stream_socket_enable_crypto | stream_socket_get_name | stream_socket_pair | stream_socket_recvfrom | stream_socket_sendto |\n  stream_socket_server | stream_socket_shutdown | stream_supports_lock | stream_wrapper_register | stream_wrapper_restore | stream_wrapper_unregister\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165706298,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  addcslashes | addslashes | bin2hex | chop | chr | chunk_split | convert_cyr_string | convert_uudecode |\n  convert_uuencode | count_chars | crc32 | crypt | echo | explode | fprintf | get_html_translation_table |\n  hebrev | hebrevc | html_entity_decode | htmlentities | htmlspecialchars | htmlspecialchars_decode | implode | join |\n  lcfirst | levenshtein | localeconv | ltrim | md5 | md5_file | metaphone | money_format |\n  nl2br | nl_langinfo | number_format | ord | parse_str | print | printf | quoted_printable_decode |\n  quoted_printable_encode | quotemeta | rtrim | setlocale | sha1 | sha1_file | similar_text | soundex |\n  sprintf | sscanf | str_getcsv | str_ireplace | str_pad | str_repeat | str_replace | str_rot13 |\n  str_shuffle | str_split | str_word_count | strcasecmp | strchr | strcmp | strcoll | strcspn |\n  strip_tags | stripcslashes | stripos | stripslashes | stristr | strlen | strnatcasecmp | strnatcmp |\n  strncasecmp | strncmp | strpbrk | strpos | strrchr | strrev | strripos | strrpos |\n  strspn | strstr | strtok | strtolower | strtoupper | strtr | substr | substr_compare |\n  substr_count | substr_replace | trim | ucfirst | ucwords | vfprintf | vprintf | vsprintf |\n  wordwrap\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255098466362,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  sybase_affected_rows | sybase_close | sybase_connect | sybase_data_seek | sybase_deadlock_retry_count | sybase_fetch_array | sybase_fetch_assoc | sybase_fetch_field |\n  sybase_fetch_object | sybase_fetch_row | sybase_field_seek | sybase_free_result | sybase_get_last_message | sybase_min_client_severity | sybase_min_error_severity | sybase_min_message_severity |\n  sybase_min_server_severity | sybase_num_fields | sybase_num_rows | sybase_pconnect | sybase_query | sybase_result | sybase_select_db | sybase_set_message_handler |\n  sybase_unbuffered_query\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165771834,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  ob_tidyhandler | tidy_access_count | tidy_clean_repair | tidy_config_count | tidy_diagnose | tidy_error_count | tidy_get_body | tidy_get_config |\n  tidy_get_error_buffer | tidy_get_head | tidy_get_html | tidy_get_html_ver | tidy_get_opt_doc | tidy_get_output | tidy_get_release | tidy_get_root |\n  tidy_get_status | tidy_getopt | tidy_is_xhtml | tidy_is_xml | tidy_load_config | tidy_parse_file | tidy_parse_string | tidy_repair_file |\n  tidy_repair_string | tidy_reset_config | tidy_save_config | tidy_set_encoding | tidy_setopt | tidy_warning_count\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165837370,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  token_get_all | token_name\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165902906,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  base64_decode | base64_encode | get_headers | get_meta_tags | http_build_query | parse_url | rawurldecode | rawurlencode |\n  urldecode | urlencode\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255121272890,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  boolval | debug_zval_dump | doubleval | empty | floatval | get_defined_vars | get_resource_type | gettype |\n  import_request_variables | intval | is_array | is_bool | is_callable | is_countable | is_double | is_float |\n  is_int | is_integer | is_iterable | is_long | is_null | is_numeric | is_object | is_real |\n  is_resource | is_scalar | is_string | isset | print_r | serialize | settype | strval |\n  unserialize | unset | var_dump | var_export\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255121403962,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  wddx_add_vars | wddx_deserialize | wddx_packet_end | wddx_packet_start | wddx_serialize_value | wddx_serialize_vars\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255165968442,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  xhprof_disable | xhprof_enable | xhprof_sample_disable | xhprof_sample_enable\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255166033978,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  utf8_decode | utf8_encode | xml_error_string | xml_get_current_byte_index | xml_get_current_column_number | xml_get_current_line_number | xml_get_error_code | xml_parse |\n  xml_parse_into_struct | xml_parser_create | xml_parser_create_ns | xml_parser_free | xml_parser_get_option | xml_parser_set_option | xml_set_character_data_handler | xml_set_default_handler |\n  xml_set_element_handler | xml_set_end_namespace_decl_handler | xml_set_external_entity_ref_handler | xml_set_notation_decl_handler | xml_set_object | xml_set_processing_instruction_handler | xml_set_start_namespace_decl_handler | xml_set_unparsed_entity_decl_handler\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255090798650,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  xmlrpc_decode | xmlrpc_decode_request | xmlrpc_encode | xmlrpc_encode_request | xmlrpc_get_type | xmlrpc_is_fault | xmlrpc_parse_method_descriptions | xmlrpc_server_add_introspection_data |\n  xmlrpc_server_call_method | xmlrpc_server_create | xmlrpc_server_destroy | xmlrpc_server_register_introspection_callback | xmlrpc_server_register_method | xmlrpc_set_type\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255166099514,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  xmlwriter_end_attribute | xmlwriter_end_cdata | xmlwriter_end_comment | xmlwriter_end_document | xmlwriter_end_dtd | xmlwriter_end_dtd_attlist | xmlwriter_end_dtd_element | xmlwriter_end_dtd_entity |\n  xmlwriter_end_element | xmlwriter_end_pi | xmlwriter_flush | xmlwriter_full_end_element | xmlwriter_open_memory | xmlwriter_open_uri | xmlwriter_output_memory | xmlwriter_set_indent |\n  xmlwriter_set_indent_string | xmlwriter_start_attribute | xmlwriter_start_attribute_ns | xmlwriter_start_cdata | xmlwriter_start_comment | xmlwriter_start_document | xmlwriter_start_dtd | xmlwriter_start_dtd_attlist |\n  xmlwriter_start_dtd_element | xmlwriter_start_dtd_entity | xmlwriter_start_element | xmlwriter_start_element_ns | xmlwriter_start_pi | xmlwriter_text | xmlwriter_write_attribute | xmlwriter_write_attribute_ns |\n  xmlwriter_write_cdata | xmlwriter_write_comment | xmlwriter_write_dtd | xmlwriter_write_dtd_attlist | xmlwriter_write_dtd_element | xmlwriter_write_dtd_entity | xmlwriter_write_element | xmlwriter_write_element_ns |\n  xmlwriter_write_pi | xmlwriter_write_raw\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255166165050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  xslt_backend_info | xslt_backend_name | xslt_backend_version | xslt_create | xslt_errno | xslt_error | xslt_free | xslt_getopt |\n  xslt_process | xslt_set_base | xslt_set_encoding | xslt_set_error_handler | xslt_set_log | xslt_set_object | xslt_set_sax_handler | xslt_set_sax_handlers |\n  xslt_set_scheme_handler | xslt_set_scheme_handlers | xslt_setopt\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255166230586,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  gzclose | gzcompress | gzdecode | gzdeflate | gzencode | gzeof | gzfile | gzgetc |\n  gzgets | gzgetss | gzinflate | gzopen | gzpassthru | gzputs | gzread | gzrewind |\n  gzseek | gztell | gzuncompress | gzwrite | readgzfile | zlib_get_coding_type\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255166296122,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  is_int | is_integer\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255114981434,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }