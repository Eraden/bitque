
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
      regex: Regex::new("\\b(asm|__asm__|auto|bool|_Bool|char|_Complex|double|float|_Imaginary|int|long|short|signed|unsigned|void|struct|union|enum)\\b"),
      scope: vec![
        Scope {
            a: 48414576463380480,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(u_char|u_short|u_int|u_long|ushort|uint|u_quad_t|quad_t|qaddr_t|caddr_t|daddr_t|dev_t|fixpt_t|blkcnt_t|blksize_t|gid_t|in_addr_t|in_port_t|ino_t|key_t|mode_t|nlink_t|id_t|pid_t|off_t|segsz_t|swblk_t|uid_t|id_t|clock_t|size_t|ssize_t|time_t|useconds_t|suseconds_t|ptrdiff_t)\\b"),
      scope: vec![
        Scope {
            a: 61925375375966221,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(pthread_attr_t|pthread_cond_t|pthread_condattr_t|pthread_mutex_t|pthread_mutexattr_t|pthread_once_t|pthread_rwlock_t|pthread_rwlockattr_t|pthread_t|pthread_key_t)\\b"),
      scope: vec![
        Scope {
            a: 61925375376031757,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(int8_t|int16_t|int32_t|int64_t|uint8_t|uint16_t|uint32_t|uint64_t|int_least8_t|int_least16_t|int_least32_t|int_least64_t|uint_least8_t|uint_least16_t|uint_least32_t|uint_least64_t|int_fast8_t|int_fast16_t|int_fast32_t|int_fast64_t|uint_fast8_t|uint_fast16_t|uint_fast32_t|uint_fast64_t|intptr_t|uintptr_t|intmax_t|intmax_t|uintmax_t|uintmax_t)\\b"),
      scope: vec![
        Scope {
            a: 61925375376097293,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(__int8|__int16|__int32|__int64)\\b"),
      scope: vec![
        Scope {
            a: 61925375376162829,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(APIENTRY|ATOM|BOOL|BOOLEAN|BYTE|CALLBACK|CCHAR|CHAR|COLORREF|CONST|DWORD|DWORDLONG|DWORD_PTR|DWORD32|DWORD64|FLOAT|HACCEL|HALF_PTR|HANDLE|HBITMAP|HBRUSH|HCOLORSPACE|HCONV|HCONVLIST|HCURSOR|HDC|HDDEDATA|HDESK|HDROP|HDWP|HENHMETAFILE|HFILE|HFONT|HGDIOBJ|HGLOBAL|HHOOK|HICON|HINSTANCE|HKEY|HKL|HLOCAL|HMENU|HMETAFILE|HMODULE|HMONITOR|HPALETTE|HPEN|HRESULT|HRGN|HRSRC|HSZ|HWINSTA|HWND|INT|INT_PTR|INT8|INT16|INT32|INT64|LANGID|LCID|LCTYPE|LGRPID|LONG|LONGLONG|LONG_PTR|LONG32|LONG64|LPARAM|LPBOOL|LPBYTE|LPCOLORREF|LPCSTR|LPCTSTR|LPCVOID|LPCWSTR|LPDWORD|LPHANDLE|LPINT|LPLONG|LPSTR|LPTSTR|LPVOID|LPWORD|LPWSTR|LRESULT|PBOOL|PBOOLEAN|PBYTE|PCHAR|PCSTR|PCTSTR|PCWSTR|PDWORD|PDWORDLONG|PDWORD_PTR|PDWORD32|PDWORD64|PFLOAT|PHALF_PTR|PHANDLE|PHKEY|PINT|PINT_PTR|PINT8|PINT16|PINT32|PINT64|PLCID|PLONG|PLONGLONG|PLONG_PTR|PLONG32|PLONG64|POINTER_32|POINTER_64|POINTER_SIGNED|POINTER_UNSIGNED|PSHORT|PSIZE_T|PSSIZE_T|PSTR|PTBYTE|PTCHAR|PTSTR|PUCHAR|PUHALF_PTR|PUINT|PUINT_PTR|PUINT8|PUINT16|PUINT32|PUINT64|PULONG|PULONGLONG|PULONG_PTR|PULONG32|PULONG64|PUSHORT|PVOID|PWCHAR|PWORD|PWSTR|QWORD|SC_HANDLE|SC_LOCK|SERVICE_STATUS_HANDLE|SHORT|SIZE_T|SSIZE_T|TBYTE|TCHAR|UCHAR|UHALF_PTR|UINT|UINT_PTR|UINT8|UINT16|UINT32|UINT64|ULONG|ULONGLONG|ULONG_PTR|ULONG32|ULONG64|UNICODE_STRING|USHORT|USN|VOID|WCHAR|WINAPI|WORD|WPARAM)\\b"),
      scope: vec![
        Scope {
            a: 61925375376228365,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(AbsoluteTime|Boolean|Byte|ByteCount|ByteOffset|BytePtr|CompTimeValue|ConstLogicalAddress|ConstStrFileNameParam|ConstStringPtr|Duration|Fixed|FixedPtr|Float32|Float32Point|Float64|Float80|Float96|FourCharCode|Fract|FractPtr|Handle|ItemCount|LogicalAddress|OptionBits|OSErr|OSStatus|OSType|OSTypePtr|PhysicalAddress|ProcessSerialNumber|ProcessSerialNumberPtr|ProcHandle|Ptr|ResType|ResTypePtr|ShortFixed|ShortFixedPtr|SignedByte|SInt16|SInt32|SInt64|SInt8|Size|StrFileName|StringHandle|StringPtr|TimeBase|TimeRecord|TimeScale|TimeValue|TimeValue64|UInt16|UInt32|UInt64|UInt8|UniChar|UniCharCount|UniCharCountPtr|UniCharPtr|UnicodeScalarValue|UniversalProcHandle|UniversalProcPtr|UnsignedFixed|UnsignedFixedPtr|UnsignedWide|UTF16Char|UTF32Char|UTF8Char)\\b"),
      scope: vec![
        Scope {
            a: 61925375375507469,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }