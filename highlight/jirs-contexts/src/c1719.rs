
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
      regex: Regex::new("\\b(new file|нов файл|fitxer nou|neue Datei|nouveau fichier|@is.po|nuovo file|새 파일|novo ficheiro|новый файл|ny fil|tập tin mới|新文件|added by them|добавени от тях|afegit per ells|von denen hinzugefügt|ajouté par eux|@is.po|@it.po|저 쪽에서 추가|adicionado por eles|добавлено ими|tillagt av dem|được thêm vào bởi họ|由他们添加|added by us|добавени от вас|afegit per nosaltres|von uns hinzugefügt|ajouté par nous|@is.po|@it.po|이 쪽에서 추가|adicionado por nós|добавлено нами|tillagt av oss|được thêm vào bởi chúng ta|由我们添加|both added|добавени и в двата случая|afegit per ambdós|von beiden hinzugefügt|ajouté de deux côtés|@is.po|@it.po|양쪽에서 추가|adicionado por ambos|оба добавлены|tillagt av bägge|được thêm vào bởi cả hai|双方添加)\\s*(:)\\s*(.*)"),
      scope: vec![
        Scope {
            a: 46446180067246103,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787055394837,
            b: 6473924464345088,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 184084724964589568,
        },
    ]),(3, vec![
        Scope {
            a: 55451949097877527,
            b: 0,
        },
        Scope {
            a: 114281327352152085,
            b: 6473924464345088,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(copied|копиран|copiat|kopiert|copié|@is.po|copiato|복사함|copiado|скопировано|kopierad|đã chép|拷贝|renamed|преименуван|canviat de nom|umbenannt|renommé|@is.po|rinominato|이름 바꿈|nome mudado|переименовано|namnbytt|đã đổi tên|重命名)\\s*(:)\\s*((.+)\\s+(->)\\s+(.+))"),
      scope: vec![
        Scope {
            a: 46446180067246103,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787055394837,
            b: 6473924464345088,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 184084724964589568,
        },
    ]),(3, vec![
        Scope {
            a: 55451949097877527,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 114281331647119572,
            b: 5911073295171584,
        },
    ]),(5, vec![
        Scope {
            a: 47288620757877339,
            b: 123286129995087872,
        },
    ]),(6, vec![
        Scope {
            a: 114281331647119572,
            b: 5911073295171584,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(typechange|смяна на вида|canviat de tipus|Typänderung|modif. type|@is.po|typechange|종류 바뀜|tipo alterado|изменен тип|typbyte|đổi-kiểu|类型变更)\\s*(:)\\s*(.*)"),
      scope: vec![
        Scope {
            a: 46446180067246103,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787055394837,
            b: 6473924464345088,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 184084724964589568,
        },
    ]),(3, vec![
        Scope {
            a: 55451949097877527,
            b: 0,
        },
        Scope {
            a: 114281331647119572,
            b: 5911073295171584,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(modified|променен|modificat|geändert|modifié|@is.po|modificato|수정함|modificado|изменено|ändrad|đã sửa|修改|both modified|променени и в двата случая|modificat per ambdós|von beiden geändert|modifié des deux côtés|@is.po|@it.po|양쪽에서 수정|modificado por ambos|оба изменены|ändrat av bägge|bị sửa bởi cả hai|双方修改)\\s*(:)\\s*(.*)"),
      scope: vec![
        Scope {
            a: 46446180067246103,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787055394837,
            b: 6473924464345088,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 184084724964589568,
        },
    ]),(3, vec![
        Scope {
            a: 55451949097877527,
            b: 0,
        },
        Scope {
            a: 114281331647120015,
            b: 5911073295171584,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(deleted|изтрит|suprimit|gelöscht|supprimé|@is.po|eliminato|삭제함|eliminado|удалено|borttagen|đã xóa|删除|deleted by them|изтрити от тях|suprimit per ells|von denen gelöscht|supprimé par eux|@is.po|@it.po|저 쪽에서 삭제|eliminado por eles|удалено ими|borttaget av dem|bị xóa đi bởi họ|由他们删除|deleted by us|изтрити от вас|suprimit per nosaltres|von uns gelöscht|supprimé par nous|@is.po|@it.po|이 쪽에서 삭제|eliminado por nós|удалено нами|borttaget av oss|bị xóa bởi chúng ta|由我们删除|both deleted|изтрити в двата случая|suprimit per ambdós|beide gelöscht|supprimé des deux côtés|@is.po|@it.po|양쪽에서 삭제|eliminado por ambos|оба удалены|borttaget av bägge|bị xóa bởi cả hai|双方删除)\\s*(:)\\s*(.*)"),
      scope: vec![
        Scope {
            a: 46446180067246103,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787055394837,
            b: 6473924464345088,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 184084724964589568,
        },
    ]),(3, vec![
        Scope {
            a: 55451949097877527,
            b: 0,
        },
        Scope {
            a: 114281335942086677,
            b: 6473924464345088,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(unknown|непозната промяна|desconegut|unbekannt|inconnu|@is.po|sconosciuto|알 수 없음|desconhecido|неизвестно|okänd|không hiểu|未知|unmerged|неслят|sense fusionar|nicht gemerged|non fusionné|@is.po|@it.po|병합하지 않음|não integrado|не слитые|osammanslagen|chưa hòa trộn|未合并)\\s*(:)\\s*(.*)"),
      scope: vec![
        Scope {
            a: 46446180067246103,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787055394837,
            b: 6473924464345088,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 184084724964589568,
        },
    ]),(3, vec![
        Scope {
            a: 55451949097877527,
            b: 0,
        },
        Scope {
            a: 114280326624772117,
            b: 6473924464345088,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }