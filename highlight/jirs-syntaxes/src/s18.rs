
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Commit".to_string(),
  file_extensions: vec!["COMMIT_EDITMSG".to_string(),"MERGE_MSG".to_string(),"TAG_EDITMSG".to_string()],
  scope: Scope { a: 281565172531200, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("hash".to_string(), "\\b\\h{7,}\\b".to_string());
    v.insert("new_file".to_string(), "new file|нов файл|fitxer nou|neue Datei|nouveau fichier|@is.po|nuovo file|새 파일|novo ficheiro|новый файл|ny fil|tập tin mới|新文件".to_string());
    v.insert("deleted".to_string(), "deleted|изтрит|suprimit|gelöscht|supprimé|@is.po|eliminato|삭제함|eliminado|удалено|borttagen|đã xóa|删除".to_string());
    v.insert("both_added".to_string(), "both added|добавени и в двата случая|afegit per ambdós|von beiden hinzugefügt|ajouté de deux côtés|@is.po|@it.po|양쪽에서 추가|adicionado por ambos|оба добавлены|tillagt av bägge|được thêm vào bởi cả hai|双方添加".to_string());
    v.insert("both_deleted".to_string(), "both deleted|изтрити в двата случая|suprimit per ambdós|beide gelöscht|supprimé des deux côtés|@is.po|@it.po|양쪽에서 삭제|eliminado por ambos|оба удалены|borttaget av bägge|bị xóa bởi cả hai|双方删除".to_string());
    v.insert("operator".to_string(), "\\b(?:drop|edit|exec|fixup|pick|reword|squash|{{shortcut}})\\b".to_string());
    v.insert("typechange".to_string(), "typechange|смяна на вида|canviat de tipus|Typänderung|modif. type|@is.po|typechange|종류 바뀜|tipo alterado|изменен тип|typbyte|đổi-kiểu|类型变更".to_string());
    v.insert("unknown".to_string(), "unknown|непозната промяна|desconegut|unbekannt|inconnu|@is.po|sconosciuto|알 수 없음|desconhecido|неизвестно|okänd|không hiểu|未知".to_string());
    v.insert("copied".to_string(), "copied|копиран|copiat|kopiert|copié|@is.po|copiato|복사함|copiado|скопировано|kopierad|đã chép|拷贝".to_string());
    v.insert("unmerged".to_string(), "unmerged|неслят|sense fusionar|nicht gemerged|non fusionné|@is.po|@it.po|병합하지 않음|não integrado|не слитые|osammanslagen|chưa hòa trộn|未合并".to_string());
    v.insert("comment_char".to_string(), "[#;]".to_string());
    v.insert("added_by_us".to_string(), "added by us|добавени от вас|afegit per nosaltres|von uns hinzugefügt|ajouté par nous|@is.po|@it.po|이 쪽에서 추가|adicionado por nós|добавлено нами|tillagt av oss|được thêm vào bởi chúng ta|由我们添加".to_string());
    v.insert("on_branch".to_string(), "On branch|На клон|En la branca|Auf Branch|Sur la branche|Sul branch|현재 브랜치|No ramo|На ветке|På grenen|Trên nhánh|位于分支".to_string());
    v.insert("modified".to_string(), "modified|променен|modificat|geändert|modifié|@is.po|modificato|수정함|modificado|изменено|ändrad|đã sửa|修改".to_string());
    v.insert("shortcut".to_string(), "[defprsx]".to_string());
    v.insert("renamed".to_string(), "renamed|преименуван|canviat de nom|umbenannt|renommé|@is.po|rinominato|이름 바꿈|nome mudado|переименовано|namnbytt|đã đổi tên|重命名".to_string());
    v.insert("added_by_them".to_string(), "added by them|добавени от тях|afegit per ells|von denen hinzugefügt|ajouté par eux|@is.po|@it.po|저 쪽에서 추가|adicionado por eles|добавлено ими|tillagt av dem|được thêm vào bởi họ|由他们添加".to_string());
    v.insert("both_modified".to_string(), "both modified|променени и в двата случая|modificat per ambdós|von beiden geändert|modifié des deux côtés|@is.po|@it.po|양쪽에서 수정|modificado por ambos|оба изменены|ändrat av bägge|bị sửa bởi cả hai|双方修改".to_string());
    v.insert("deleted_by_them".to_string(), "deleted by them|изтрити от тях|suprimit per ells|von denen gelöscht|supprimé par eux|@is.po|@it.po|저 쪽에서 삭제|eliminado por eles|удалено ими|borttaget av dem|bị xóa đi bởi họ|由他们删除".to_string());
    v.insert("date".to_string(), "Date|Дата|Data|Datum|Date|@is.po|@it.po|시각|Data|Дата|Datum|Ngày tháng|日期".to_string());
    v.insert("deleted_by_us".to_string(), "deleted by us|изтрити от вас|suprimit per nosaltres|von uns gelöscht|supprimé par nous|@is.po|@it.po|이 쪽에서 삭제|eliminado por nós|удалено нами|borttaget av oss|bị xóa bởi chúng ta|由我们删除".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_comments_0".to_string(), ContextId { index: 1713 });
    v.insert("dropped-content".to_string(), ContextId { index: 1726 });
    v.insert("branch-line".to_string(), ContextId { index: 1718 });
    v.insert("comments".to_string(), ContextId { index: 1721 });
    v.insert("#anon_commit-message_0".to_string(), ContextId { index: 1714 });
    v.insert("heading".to_string(), ContextId { index: 1727 });
    v.insert("change-list".to_string(), ContextId { index: 1719 });
    v.insert("__main".to_string(), ContextId { index: 1716 });
    v.insert("#anon_dropped-content_0".to_string(), ContextId { index: 1715 });
    v.insert("__start".to_string(), ContextId { index: 1717 });
    v.insert("commit-subject".to_string(), ContextId { index: 1724 });
    v.insert("date-line".to_string(), ContextId { index: 1725 });
    v.insert("commands-line".to_string(), ContextId { index: 1720 });
    v.insert("commit-separator".to_string(), ContextId { index: 1723 });
    v.insert("prototype".to_string(), ContextId { index: 1729 });
    v.insert("signed-off".to_string(), ContextId { index: 1730 });
    v.insert("main".to_string(), ContextId { index: 1728 });
    v.insert("commit-message".to_string(), ContextId { index: 1722 });
    v
  }
} }