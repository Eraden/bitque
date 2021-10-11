use std::str::FromStr;

pub fn is_email(s: &str) -> bool {
    let mut has_at = false;
    let mut has_dot = false;

    for c in s.chars() {
        match c {
            '@' if !has_at => {
                has_at = true;
            }
            '.' if has_at => {
                has_dot = true;
            }
            '\n' | ' ' | '\t' | '\r' => return false,
            '@' if has_at => return false,
            _ if has_dot => return true,
            _ => (),
        }
    }
    false
}

pub fn is_token(s: &str) -> bool {
    uuid::Uuid::from_str(s).is_ok()
}
