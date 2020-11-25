use crate::syntect::highlighting::{Style, StyleModifier};
use crate::syntect::ScopeStackOp;
use std::fmt::Write;
use std::ops::Range;

pub fn as_24_bit_terminal_escaped(v: &[(Style, &str)], bg: bool) -> String {
    let mut s: String = String::new();
    for &(ref style, text) in v.iter() {
        if bg {
            write!(
                s,
                "\x1b[48;2;{};{};{}m",
                style.background.r, style.background.g, style.background.b
            )
            .unwrap();
        }
        write!(
            s,
            "\x1b[38;2;{};{};{}m{}",
            style.foreground.r, style.foreground.g, style.foreground.b, text
        )
        .unwrap();
    }
    // s.push_str("\x1b[0m");
    s
}

const LATEX_REPLACE: [(&'static str, &'static str); 3] =
    [("\\", "\\\\"), ("{", "\\{"), ("}", "\\}")];

pub fn as_latex_escaped(v: &[(Style, &str)]) -> String {
    let mut s: String = String::new();
    let mut prev_style: Option<Style> = None;
    let mut content: String;
    fn textcolor(style: &Style, first: bool) -> String {
        format!(
            "{}\\textcolor[RGB]{{{},{},{}}}{{",
            if first { "" } else { "}" },
            style.foreground.r,
            style.foreground.b,
            style.foreground.g
        )
    }
    for &(style, text) in v.iter() {
        if let Some(ps) = prev_style {
            match text {
                " " => {
                    s.push(' ');
                    continue;
                }
                "\n" => continue,
                _ => (),
            }
            if style != ps {
                write!(s, "{}", textcolor(&style, false)).unwrap();
            }
        } else {
            write!(s, "{}", textcolor(&style, true)).unwrap();
        }
        content = text.to_string();
        for &(old, new) in LATEX_REPLACE.iter() {
            content = content.replace(&old, &new);
        }
        write!(s, "{}", &content).unwrap();
        prev_style = Some(style);
    }
    s.push('}');
    s
}

pub fn debug_print_ops(line: &str, ops: &[(usize, ScopeStackOp)]) {
    for &(i, ref op) in ops.iter() {
        println!("{}", line.trim_end());
        print!("{: <1$}", "", i);
        match *op {
            ScopeStackOp::Push(s) => {
                println!("^ +{}", s);
            }
            ScopeStackOp::Pop(count) => {
                println!("^ pop {}", count);
            }
            ScopeStackOp::Clear(amount) => {
                println!("^ clear {:?}", amount);
            }
            ScopeStackOp::Restore => println!("^ restore"),
            ScopeStackOp::Noop => println!("noop"),
        }
    }
}

pub struct LinesWithEndings<'a> {
    input: &'a str,
}

impl<'a> LinesWithEndings<'a> {
    pub fn from(input: &'a str) -> LinesWithEndings<'a> {
        LinesWithEndings { input }
    }
}

impl<'a> Iterator for LinesWithEndings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.input.is_empty() {
            return None;
        }
        let split = self
            .input
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or_else(|| self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}

pub fn split_at<'a, A: Clone>(
    v: &[(A, &'a str)],
    split_i: usize,
) -> (Vec<(A, &'a str)>, Vec<(A, &'a str)>) {
    // This function works by gradually reducing the problem into smaller sub-problems from the front
    let mut rest = v;
    let mut rest_split_i = split_i;

    // Consume all tokens before the split
    let mut before = Vec::new();
    for tok in rest {
        // Use for instead of a while to avoid bounds checks
        if tok.1.len() > rest_split_i {
            break;
        }
        before.push(tok.clone());
        rest_split_i -= tok.1.len();
    }
    rest = &rest[before.len()..];

    let mut after = Vec::new();
    // If necessary, split the token the split falls inside
    if !rest.is_empty() && rest_split_i > 0 {
        let (sa, sb) = rest[0].1.split_at(rest_split_i);
        before.push((rest[0].0.clone(), sa));
        after.push((rest[0].0.clone(), sb));
        rest = &rest[1..];
    }

    after.extend_from_slice(rest);

    (before, after)
}
