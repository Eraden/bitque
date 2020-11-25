use jirs_syntect::*;
#[cfg(feature = "Plain Text")]
pub mod s0;
#[cfg(feature = "ASP")]
pub mod s1;
#[cfg(feature = "HTML (ASP)")]
pub mod s2;
#[cfg(feature = "ActionScript")]
pub mod s3;
#[cfg(feature = "AppleScript")]
pub mod s4;
#[cfg(feature = "Batch File")]
pub mod s5;
#[cfg(feature = "NAnt Build File")]
pub mod s6;
#[cfg(feature = "C#")]
pub mod s7;
#[cfg(feature = "C++")]
pub mod s8;
#[cfg(feature = "C")]
pub mod s9;
#[cfg(feature = "CSS")]
pub mod s10;
#[cfg(feature = "Clojure")]
pub mod s11;
#[cfg(feature = "D")]
pub mod s12;
#[cfg(feature = "DMD Output")]
pub mod s13;
#[cfg(feature = "Diff")]
pub mod s14;
#[cfg(feature = "Erlang")]
pub mod s15;
#[cfg(feature = "HTML (Erlang)")]
pub mod s16;
#[cfg(feature = "Git Attributes")]
pub mod s17;
#[cfg(feature = "Git Commit")]
pub mod s18;
#[cfg(feature = "Git Common")]
pub mod s19;
#[cfg(feature = "Git Config")]
pub mod s20;
#[cfg(feature = "Git Ignore")]
pub mod s21;
#[cfg(feature = "Git Link")]
pub mod s22;
#[cfg(feature = "Git Log")]
pub mod s23;
#[cfg(feature = "Git Mailmap")]
pub mod s24;
#[cfg(feature = "Git Rebase Todo")]
pub mod s25;
#[cfg(feature = "Go")]
pub mod s26;
#[cfg(feature = "Graphviz (DOT)")]
pub mod s27;
#[cfg(feature = "Groovy")]
pub mod s28;
#[cfg(feature = "HTML")]
pub mod s29;
#[cfg(feature = "Haskell")]
pub mod s30;
#[cfg(feature = "Literate Haskell")]
pub mod s31;
#[cfg(feature = "JSON")]
pub mod s32;
#[cfg(feature = "Java Server Page (JSP)")]
pub mod s33;
#[cfg(feature = "Java")]
pub mod s34;
#[cfg(feature = "Javadoc")]
pub mod s35;
#[cfg(feature = "Java Properties")]
pub mod s36;
#[cfg(feature = "JavaScript")]
pub mod s37;
#[cfg(feature = "Regular Expressions (Javascript)")]
pub mod s38;
#[cfg(feature = "BibTeX")]
pub mod s39;
#[cfg(feature = "LaTeX Log")]
pub mod s40;
#[cfg(feature = "LaTeX")]
pub mod s41;
#[cfg(feature = "TeX")]
pub mod s42;
#[cfg(feature = "Lisp")]
pub mod s43;
#[cfg(feature = "Lua")]
pub mod s44;
#[cfg(feature = "Make Output")]
pub mod s45;
#[cfg(feature = "Makefile")]
pub mod s46;
#[cfg(feature = "Markdown")]
pub mod s47;
#[cfg(feature = "MultiMarkdown")]
pub mod s48;
#[cfg(feature = "MATLAB")]
pub mod s49;
#[cfg(feature = "OCaml")]
pub mod s50;
#[cfg(feature = "OCamllex")]
pub mod s51;
#[cfg(feature = "OCamlyacc")]
pub mod s52;
#[cfg(feature = "camlp4")]
pub mod s53;
#[cfg(feature = "Objective-C++")]
pub mod s54;
#[cfg(feature = "Objective-C")]
pub mod s55;
#[cfg(feature = "PHP Source")]
pub mod s56;
#[cfg(feature = "PHP")]
pub mod s57;
#[cfg(feature = "Regular Expressions (PHP)")]
pub mod s58;
#[cfg(feature = "Pascal")]
pub mod s59;
#[cfg(feature = "Perl")]
pub mod s60;
#[cfg(feature = "Python")]
pub mod s61;
#[cfg(feature = "Regular Expressions (Python)")]
pub mod s62;
#[cfg(feature = "R Console")]
pub mod s63;
#[cfg(feature = "R")]
pub mod s64;
#[cfg(feature = "Rd (R Documentation)")]
pub mod s65;
#[cfg(feature = "HTML (Rails)")]
pub mod s66;
#[cfg(feature = "JavaScript (Rails)")]
pub mod s67;
#[cfg(feature = "Ruby Haml")]
pub mod s68;
#[cfg(feature = "Ruby on Rails")]
pub mod s69;
#[cfg(feature = "SQL (Rails)")]
pub mod s70;
#[cfg(feature = "Regular Expression")]
pub mod s71;
#[cfg(feature = "reStructuredText")]
pub mod s72;
#[cfg(feature = "Ruby")]
pub mod s73;
#[cfg(feature = "Cargo Build Results")]
pub mod s74;
#[cfg(feature = "Rust")]
pub mod s75;
#[cfg(feature = "SQL")]
pub mod s76;
#[cfg(feature = "Scala")]
pub mod s77;
#[cfg(feature = "Bourne Again Shell (bash)")]
pub mod s78;
#[cfg(feature = "Shell-Unix-Generic")]
pub mod s79;
#[cfg(feature = "commands-builtin-shell-bash")]
pub mod s80;
#[cfg(feature = "HTML (Tcl)")]
pub mod s81;
#[cfg(feature = "Tcl")]
pub mod s82;
#[cfg(feature = "Textile")]
pub mod s83;
#[cfg(feature = "XML")]
pub mod s84;
#[cfg(feature = "YAML")]
pub mod s85;
#[cfg(feature = "AWK")]
pub mod s86;
#[cfg(feature = "Apache Conf")]
pub mod s87;
#[cfg(feature = "AsciiDoc (Asciidoctor)")]
pub mod s88;
#[cfg(feature = "ARM Assembly")]
pub mod s89;
#[cfg(feature = "Assembly (x86_64)")]
pub mod s90;
#[cfg(feature = "CMake C Header")]
pub mod s91;
#[cfg(feature = "CMake C++ Header")]
pub mod s92;
#[cfg(feature = "CMake")]
pub mod s93;
#[cfg(feature = "CMakeCache")]
pub mod s94;
#[cfg(feature = "CMakeCommands")]
pub mod s95;
#[cfg(feature = "Advanced CSV")]
pub mod s96;
#[cfg(feature = "Cabal")]
pub mod s97;
#[cfg(feature = "CoffeeScript")]
pub mod s98;
#[cfg(feature = "CpuInfo")]
pub mod s99;
#[cfg(feature = "Crystal")]
pub mod s100;
#[cfg(feature = "Dart Analysis Output")]
pub mod s101;
#[cfg(feature = "Dart")]
pub mod s102;
#[cfg(feature = "Dockerfile")]
pub mod s103;
#[cfg(feature = "DotENV")]
pub mod s104;
#[cfg(feature = "Elixir")]
pub mod s105;
#[cfg(feature = "HTML (EEx)")]
pub mod s106;
#[cfg(feature = "Regular Expressions (Elixir)")]
pub mod s107;
#[cfg(feature = "Elm Compile Messages")]
pub mod s108;
#[cfg(feature = "Elm Documentation")]
pub mod s109;
#[cfg(feature = "Elm")]
pub mod s110;
#[cfg(feature = "Email")]
pub mod s111;
#[cfg(feature = "F#")]
pub mod s112;
#[cfg(feature = "Friendly Interactive Shell (fish)")]
pub mod s113;
#[cfg(feature = "Fortran (Fixed Form)")]
pub mod s114;
#[cfg(feature = "Fortran (Modern)")]
pub mod s115;
#[cfg(feature = "Fortran Namelist")]
pub mod s116;
#[cfg(feature = "GFortran Build Results")]
pub mod s117;
#[cfg(feature = "OpenMP (Fortran)")]
pub mod s118;
#[cfg(feature = "fstab")]
pub mod s119;
#[cfg(feature = "GLSL")]
pub mod s120;
#[cfg(feature = "GraphQL")]
pub mod s121;
#[cfg(feature = "group")]
pub mod s122;
#[cfg(feature = "HTML (Twig)")]
pub mod s123;
#[cfg(feature = "hosts")]
pub mod s124;
#[cfg(feature = "INI")]
pub mod s125;
#[cfg(feature = "JavaScript (Babel)")]
pub mod s126;
#[cfg(feature = "HTML (Jinja2)")]
pub mod s127;
#[cfg(feature = "Jinja2")]
pub mod s128;
#[cfg(feature = "jsonnet")]
pub mod s129;
#[cfg(feature = "Julia")]
pub mod s130;
#[cfg(feature = "Kotlin")]
pub mod s131;
#[cfg(feature = "Less")]
pub mod s132;
#[cfg(feature = "Manpage")]
pub mod s133;
#[cfg(feature = "MemInfo")]
pub mod s134;
#[cfg(feature = "nginx")]
pub mod s135;
#[cfg(feature = "Nim")]
pub mod s136;
#[cfg(feature = "Nix")]
pub mod s137;
#[cfg(feature = "orgmode")]
pub mod s138;
#[cfg(feature = "passwd")]
pub mod s139;
#[cfg(feature = "PowerShell")]
pub mod s140;
#[cfg(feature = "Protocol Buffer")]
pub mod s141;
#[cfg(feature = "Protocol Buffer (TEXT)")]
pub mod s142;
#[cfg(feature = "Puppet")]
pub mod s143;
#[cfg(feature = "PureScript")]
pub mod s144;
#[cfg(feature = "QML")]
pub mod s145;
#[cfg(feature = "Rego")]
pub mod s146;
#[cfg(feature = "resolv")]
pub mod s147;
#[cfg(feature = "Robot Framework syntax highlighting.")]
pub mod s148;
#[cfg(feature = "SCSS")]
pub mod s149;
#[cfg(feature = "Sass")]
pub mod s150;
#[cfg(feature = "Salt State (SLS)")]
pub mod s151;
#[cfg(feature = "SML")]
pub mod s152;
#[cfg(feature = "Strace")]
pub mod s153;
#[cfg(feature = "Stylus")]
pub mod s154;
#[cfg(feature = "Swift")]
pub mod s155;
#[cfg(feature = "syslog")]
pub mod s156;
#[cfg(feature = "TOML")]
pub mod s157;
#[cfg(feature = "JSON (Terraform)")]
pub mod s158;
#[cfg(feature = "Terraform")]
pub mod s159;
#[cfg(feature = "TypeScript")]
pub mod s160;
#[cfg(feature = "TypeScriptReact")]
pub mod s161;
#[cfg(feature = "Verilog")]
pub mod s162;
#[cfg(feature = "VimL")]
pub mod s163;
#[cfg(feature = "Vue Component")]
pub mod s164;
#[cfg(feature = "requirements.txt")]
pub mod s165;
#[cfg(feature = "Highlight non-printables")]
pub mod s166;
#[cfg(feature = "Authorized Keys")]
pub mod s167;
#[cfg(feature = "Known Hosts")]
pub mod s168;
#[cfg(feature = "Private Key")]
pub mod s169;
#[cfg(feature = "SSH Common")]
pub mod s170;
#[cfg(feature = "SSH Config")]
pub mod s171;
#[cfg(feature = "SSH Crypto")]
pub mod s172;
#[cfg(feature = "SSHD Config")]
pub mod s173;
#[cfg(feature = "varlink")]
pub mod s174;
pub fn load() -> Vec<SyntaxReference> {
  vec![
    #[cfg(feature = "Plain Text")]
    s0::load(),
    #[cfg(feature = "ASP")]
    s1::load(),
    #[cfg(feature = "HTML (ASP)")]
    s2::load(),
    #[cfg(feature = "ActionScript")]
    s3::load(),
    #[cfg(feature = "AppleScript")]
    s4::load(),
    #[cfg(feature = "Batch File")]
    s5::load(),
    #[cfg(feature = "NAnt Build File")]
    s6::load(),
    #[cfg(feature = "C#")]
    s7::load(),
    #[cfg(feature = "C++")]
    s8::load(),
    #[cfg(feature = "C")]
    s9::load(),
    #[cfg(feature = "CSS")]
    s10::load(),
    #[cfg(feature = "Clojure")]
    s11::load(),
    #[cfg(feature = "D")]
    s12::load(),
    #[cfg(feature = "DMD Output")]
    s13::load(),
    #[cfg(feature = "Diff")]
    s14::load(),
    #[cfg(feature = "Erlang")]
    s15::load(),
    #[cfg(feature = "HTML (Erlang)")]
    s16::load(),
    #[cfg(feature = "Git Attributes")]
    s17::load(),
    #[cfg(feature = "Git Commit")]
    s18::load(),
    #[cfg(feature = "Git Common")]
    s19::load(),
    #[cfg(feature = "Git Config")]
    s20::load(),
    #[cfg(feature = "Git Ignore")]
    s21::load(),
    #[cfg(feature = "Git Link")]
    s22::load(),
    #[cfg(feature = "Git Log")]
    s23::load(),
    #[cfg(feature = "Git Mailmap")]
    s24::load(),
    #[cfg(feature = "Git Rebase Todo")]
    s25::load(),
    #[cfg(feature = "Go")]
    s26::load(),
    #[cfg(feature = "Graphviz (DOT)")]
    s27::load(),
    #[cfg(feature = "Groovy")]
    s28::load(),
    #[cfg(feature = "HTML")]
    s29::load(),
    #[cfg(feature = "Haskell")]
    s30::load(),
    #[cfg(feature = "Literate Haskell")]
    s31::load(),
    #[cfg(feature = "JSON")]
    s32::load(),
    #[cfg(feature = "Java Server Page (JSP)")]
    s33::load(),
    #[cfg(feature = "Java")]
    s34::load(),
    #[cfg(feature = "Javadoc")]
    s35::load(),
    #[cfg(feature = "Java Properties")]
    s36::load(),
    #[cfg(feature = "JavaScript")]
    s37::load(),
    #[cfg(feature = "Regular Expressions (Javascript)")]
    s38::load(),
    #[cfg(feature = "BibTeX")]
    s39::load(),
    #[cfg(feature = "LaTeX Log")]
    s40::load(),
    #[cfg(feature = "LaTeX")]
    s41::load(),
    #[cfg(feature = "TeX")]
    s42::load(),
    #[cfg(feature = "Lisp")]
    s43::load(),
    #[cfg(feature = "Lua")]
    s44::load(),
    #[cfg(feature = "Make Output")]
    s45::load(),
    #[cfg(feature = "Makefile")]
    s46::load(),
    #[cfg(feature = "Markdown")]
    s47::load(),
    #[cfg(feature = "MultiMarkdown")]
    s48::load(),
    #[cfg(feature = "MATLAB")]
    s49::load(),
    #[cfg(feature = "OCaml")]
    s50::load(),
    #[cfg(feature = "OCamllex")]
    s51::load(),
    #[cfg(feature = "OCamlyacc")]
    s52::load(),
    #[cfg(feature = "camlp4")]
    s53::load(),
    #[cfg(feature = "Objective-C++")]
    s54::load(),
    #[cfg(feature = "Objective-C")]
    s55::load(),
    #[cfg(feature = "PHP Source")]
    s56::load(),
    #[cfg(feature = "PHP")]
    s57::load(),
    #[cfg(feature = "Regular Expressions (PHP)")]
    s58::load(),
    #[cfg(feature = "Pascal")]
    s59::load(),
    #[cfg(feature = "Perl")]
    s60::load(),
    #[cfg(feature = "Python")]
    s61::load(),
    #[cfg(feature = "Regular Expressions (Python)")]
    s62::load(),
    #[cfg(feature = "R Console")]
    s63::load(),
    #[cfg(feature = "R")]
    s64::load(),
    #[cfg(feature = "Rd (R Documentation)")]
    s65::load(),
    #[cfg(feature = "HTML (Rails)")]
    s66::load(),
    #[cfg(feature = "JavaScript (Rails)")]
    s67::load(),
    #[cfg(feature = "Ruby Haml")]
    s68::load(),
    #[cfg(feature = "Ruby on Rails")]
    s69::load(),
    #[cfg(feature = "SQL (Rails)")]
    s70::load(),
    #[cfg(feature = "Regular Expression")]
    s71::load(),
    #[cfg(feature = "reStructuredText")]
    s72::load(),
    #[cfg(feature = "Ruby")]
    s73::load(),
    #[cfg(feature = "Cargo Build Results")]
    s74::load(),
    #[cfg(feature = "Rust")]
    s75::load(),
    #[cfg(feature = "SQL")]
    s76::load(),
    #[cfg(feature = "Scala")]
    s77::load(),
    #[cfg(feature = "Bourne Again Shell (bash)")]
    s78::load(),
    #[cfg(feature = "Shell-Unix-Generic")]
    s79::load(),
    #[cfg(feature = "commands-builtin-shell-bash")]
    s80::load(),
    #[cfg(feature = "HTML (Tcl)")]
    s81::load(),
    #[cfg(feature = "Tcl")]
    s82::load(),
    #[cfg(feature = "Textile")]
    s83::load(),
    #[cfg(feature = "XML")]
    s84::load(),
    #[cfg(feature = "YAML")]
    s85::load(),
    #[cfg(feature = "AWK")]
    s86::load(),
    #[cfg(feature = "Apache Conf")]
    s87::load(),
    #[cfg(feature = "AsciiDoc (Asciidoctor)")]
    s88::load(),
    #[cfg(feature = "ARM Assembly")]
    s89::load(),
    #[cfg(feature = "Assembly (x86_64)")]
    s90::load(),
    #[cfg(feature = "CMake C Header")]
    s91::load(),
    #[cfg(feature = "CMake C++ Header")]
    s92::load(),
    #[cfg(feature = "CMake")]
    s93::load(),
    #[cfg(feature = "CMakeCache")]
    s94::load(),
    #[cfg(feature = "CMakeCommands")]
    s95::load(),
    #[cfg(feature = "Advanced CSV")]
    s96::load(),
    #[cfg(feature = "Cabal")]
    s97::load(),
    #[cfg(feature = "CoffeeScript")]
    s98::load(),
    #[cfg(feature = "CpuInfo")]
    s99::load(),
    #[cfg(feature = "Crystal")]
    s100::load(),
    #[cfg(feature = "Dart Analysis Output")]
    s101::load(),
    #[cfg(feature = "Dart")]
    s102::load(),
    #[cfg(feature = "Dockerfile")]
    s103::load(),
    #[cfg(feature = "DotENV")]
    s104::load(),
    #[cfg(feature = "Elixir")]
    s105::load(),
    #[cfg(feature = "HTML (EEx)")]
    s106::load(),
    #[cfg(feature = "Regular Expressions (Elixir)")]
    s107::load(),
    #[cfg(feature = "Elm Compile Messages")]
    s108::load(),
    #[cfg(feature = "Elm Documentation")]
    s109::load(),
    #[cfg(feature = "Elm")]
    s110::load(),
    #[cfg(feature = "Email")]
    s111::load(),
    #[cfg(feature = "F#")]
    s112::load(),
    #[cfg(feature = "Friendly Interactive Shell (fish)")]
    s113::load(),
    #[cfg(feature = "Fortran (Fixed Form)")]
    s114::load(),
    #[cfg(feature = "Fortran (Modern)")]
    s115::load(),
    #[cfg(feature = "Fortran Namelist")]
    s116::load(),
    #[cfg(feature = "GFortran Build Results")]
    s117::load(),
    #[cfg(feature = "OpenMP (Fortran)")]
    s118::load(),
    #[cfg(feature = "fstab")]
    s119::load(),
    #[cfg(feature = "GLSL")]
    s120::load(),
    #[cfg(feature = "GraphQL")]
    s121::load(),
    #[cfg(feature = "group")]
    s122::load(),
    #[cfg(feature = "HTML (Twig)")]
    s123::load(),
    #[cfg(feature = "hosts")]
    s124::load(),
    #[cfg(feature = "INI")]
    s125::load(),
    #[cfg(feature = "JavaScript (Babel)")]
    s126::load(),
    #[cfg(feature = "HTML (Jinja2)")]
    s127::load(),
    #[cfg(feature = "Jinja2")]
    s128::load(),
    #[cfg(feature = "jsonnet")]
    s129::load(),
    #[cfg(feature = "Julia")]
    s130::load(),
    #[cfg(feature = "Kotlin")]
    s131::load(),
    #[cfg(feature = "Less")]
    s132::load(),
    #[cfg(feature = "Manpage")]
    s133::load(),
    #[cfg(feature = "MemInfo")]
    s134::load(),
    #[cfg(feature = "nginx")]
    s135::load(),
    #[cfg(feature = "Nim")]
    s136::load(),
    #[cfg(feature = "Nix")]
    s137::load(),
    #[cfg(feature = "orgmode")]
    s138::load(),
    #[cfg(feature = "passwd")]
    s139::load(),
    #[cfg(feature = "PowerShell")]
    s140::load(),
    #[cfg(feature = "Protocol Buffer")]
    s141::load(),
    #[cfg(feature = "Protocol Buffer (TEXT)")]
    s142::load(),
    #[cfg(feature = "Puppet")]
    s143::load(),
    #[cfg(feature = "PureScript")]
    s144::load(),
    #[cfg(feature = "QML")]
    s145::load(),
    #[cfg(feature = "Rego")]
    s146::load(),
    #[cfg(feature = "resolv")]
    s147::load(),
    #[cfg(feature = "Robot Framework syntax highlighting.")]
    s148::load(),
    #[cfg(feature = "SCSS")]
    s149::load(),
    #[cfg(feature = "Sass")]
    s150::load(),
    #[cfg(feature = "Salt State (SLS)")]
    s151::load(),
    #[cfg(feature = "SML")]
    s152::load(),
    #[cfg(feature = "Strace")]
    s153::load(),
    #[cfg(feature = "Stylus")]
    s154::load(),
    #[cfg(feature = "Swift")]
    s155::load(),
    #[cfg(feature = "syslog")]
    s156::load(),
    #[cfg(feature = "TOML")]
    s157::load(),
    #[cfg(feature = "JSON (Terraform)")]
    s158::load(),
    #[cfg(feature = "Terraform")]
    s159::load(),
    #[cfg(feature = "TypeScript")]
    s160::load(),
    #[cfg(feature = "TypeScriptReact")]
    s161::load(),
    #[cfg(feature = "Verilog")]
    s162::load(),
    #[cfg(feature = "VimL")]
    s163::load(),
    #[cfg(feature = "Vue Component")]
    s164::load(),
    #[cfg(feature = "requirements.txt")]
    s165::load(),
    #[cfg(feature = "Highlight non-printables")]
    s166::load(),
    #[cfg(feature = "Authorized Keys")]
    s167::load(),
    #[cfg(feature = "Known Hosts")]
    s168::load(),
    #[cfg(feature = "Private Key")]
    s169::load(),
    #[cfg(feature = "SSH Common")]
    s170::load(),
    #[cfg(feature = "SSH Config")]
    s171::load(),
    #[cfg(feature = "SSH Crypto")]
    s172::load(),
    #[cfg(feature = "SSHD Config")]
    s173::load(),
    #[cfg(feature = "varlink")]
    s174::load(),
  ]
}
