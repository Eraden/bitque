
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
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 46444217273745408,
            b: 0,
        },
        Scope {
            a: 55451420828565625,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 34058472181989376,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7939 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(b)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439225,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217273745408,
            b: 0,
        },
        Scope {
            a: 55451420828565625,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 34058472181989376,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7940 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(r)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439225,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217273745408,
            b: 0,
        },
        Scope {
            a: 55451420828565625,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 34058472181989376,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7941 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(v|raw|(?:[^\\s(?:[(){}\\[\\],.;:\'\"`@#])(?:[-+*/\\\\=^:<>~?&$%|!]|[≤≥¬←→↔↚↛↠↣↦↮⇎⇏⇒⇔⇴⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿⟵⟶⟷⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤌⤍⤎⤏⤐⤑⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥊⥋⥎⥐⥒⥓⥖⥗⥚⥛⥞⥟⥢⥤⥦⥧⥨⥩⥪⥫⥬⥭⥰⧴⬱⬰⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￫≡≠≢∈∉∋∌⊆⊈⊂⊄⊊∝∊∍∥∦∷∺∻∽∾≁≃≄≅≆≇≈≉≊≋≌≍≎≐≑≒≓≔≕≖≗≘≙≚≛≜≝≞≟≣≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊃⊅⊇⊉⊋⊏⊐⊑⊒⊜⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⋍⋐⋑⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⟈⟉⟒⦷⧀⧁⧡⧣⧤⧥⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫷⫸⫹⫺⊢⊣⊕⊖⊞⊟∪∨⊔±∓∔∸≂≏⊎⊻⊽⋎⋓⧺⧻⨈⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨹⨺⩁⩂⩅⩊⩌⩏⩐⩒⩔⩖⩗⩛⩝⩡⩢⩣÷⋅∘×∩∧⊗⊘⊙⊚⊛⊠⊡⊓∗∙∤⅋≀⊼⋄⋆⋇⋉⋊⋋⋌⋏⋒⟑⦸⦼⦾⦿⧶⧷⨇⨰⨱⨲⨳⨴⨵⨶⨷⨸⨻⨼⨽⩀⩃⩄⩋⩍⩎⩑⩓⩕⩘⩚⩜⩞⩟⩠⫛⊍▷⨝⟕⟖⟗↑↓⇵⟰⟱⤈⤉⤊⤋⤒⤓⥉⥌⥍⥏⥑⥔⥕⥘⥙⥜⥝⥠⥡⥣⥥⥮⥯￪￬])0-9](?:[^\\s(?:[(){}\\[\\],.;:\'\"`@#])(?:[-+*/\\\\=^:<>~?&$%|!]|[≤≥¬←→↔↚↛↠↣↦↮⇎⇏⇒⇔⇴⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿⟵⟶⟷⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤌⤍⤎⤏⤐⤑⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥊⥋⥎⥐⥒⥓⥖⥗⥚⥛⥞⥟⥢⥤⥦⥧⥨⥩⥪⥫⥬⥭⥰⧴⬱⬰⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￫≡≠≢∈∉∋∌⊆⊈⊂⊄⊊∝∊∍∥∦∷∺∻∽∾≁≃≄≅≆≇≈≉≊≋≌≍≎≐≑≒≓≔≕≖≗≘≙≚≛≜≝≞≟≣≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊃⊅⊇⊉⊋⊏⊐⊑⊒⊜⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⋍⋐⋑⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⟈⟉⟒⦷⧀⧁⧡⧣⧤⧥⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫷⫸⫹⫺⊢⊣⊕⊖⊞⊟∪∨⊔±∓∔∸≂≏⊎⊻⊽⋎⋓⧺⧻⨈⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨹⨺⩁⩂⩅⩊⩌⩏⩐⩒⩔⩖⩗⩛⩝⩡⩢⩣÷⋅∘×∩∧⊗⊘⊙⊚⊛⊠⊡⊓∗∙∤⅋≀⊼⋄⋆⋇⋉⋊⋋⋌⋏⋒⟑⦸⦼⦾⦿⧶⧷⨇⨰⨱⨲⨳⨴⨵⨶⨷⨸⨻⨼⨽⩀⩃⩄⩋⩍⩎⩑⩓⩕⩘⩚⩜⩞⩟⩠⫛⊍▷⨝⟕⟖⟗↑↓⇵⟰⟱⤈⤉⤊⤋⤒⤓⥉⥌⥍⥏⥑⥔⥕⥘⥙⥜⥝⥠⥡⥣⥥⥮⥯￪￬])]|!)*))(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439225,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217273745408,
            b: 0,
        },
        Scope {
            a: 55451420828565625,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 34058472181989376,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7943 }),
    ]),
      with_prototype: None
    }),
]
} }