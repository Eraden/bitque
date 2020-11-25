use super::regex::Regex;
use super::scope::*;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub enum ParseSyntaxError {
    /// The file must contain at least one YAML document
    EmptyFile,
    /// Some keys are required for something to be a valid `.sublime-syntax`
    MissingMandatoryKey(&'static str),
    /// Invalid regex
    RegexCompileError(String, Box<dyn Error + Send>),
    /// A scope that syntect's scope implementation can't handle
    InvalidScope(ParseScopeError),
    /// A reference to another file that is invalid
    BadFileRef,
    /// Syntaxes must have a context named "main"
    MainMissing,
    /// Some part of the YAML file is the wrong type (e.g a string but should be a list)
    /// Sorry this doesn't give you any way to narrow down where this is.
    /// Maybe use Sublime Text to figure it out.
    TypeMismatch,
}

fn str_to_scopes(s: &str, repo: &mut ScopeRepository) -> Result<Vec<Scope>, ParseSyntaxError> {
    s.split_whitespace()
        .map(|scope| repo.build(scope).map_err(ParseSyntaxError::InvalidScope))
        .collect()
}

struct ParserState<'a> {
    scope_repo: &'a mut ScopeRepository,
    variables: HashMap<String, String>,
    variable_regex: Regex,
    backref_regex: Regex,
    lines_include_newline: bool,
}

// `__start` must not include prototypes from the actual syntax definition,
// otherwise it's possible that a prototype makes us pop out of `__start`.
/*
impl<'l> SyntaxDefinition<'l> {
    fn parse_context(
        vec: &[Yaml],
        // TODO: Maybe just pass the scope repo if that's all that's needed?
        state: &mut ParserState<'_>,
        contexts: &mut HashMap<String, Context>,
        is_prototype: bool,
        namer: &mut ContextNamer,
    ) -> Result<String, ParseSyntaxError> {
        let mut context = Context::new(!is_prototype);
        let name = namer.next();

        for y in vec.iter() {
            let map = y.as_hash().ok_or(ParseSyntaxError::TypeMismatch)?;

            let mut is_special = false;
            if let Ok(x) = get_key(map, "meta_scope", |x| x.as_str()) {
                context.meta_scope = str_to_scopes(x, state.scope_repo)?;
                is_special = true;
            }
            if let Ok(x) = get_key(map, "meta_content_scope", |x| x.as_str()) {
                context.meta_content_scope = str_to_scopes(x, state.scope_repo)?;
                is_special = true;
            }
            if let Ok(x) = get_key(map, "meta_include_prototype", |x| x.as_bool()) {
                context.meta_include_prototype = x;
                is_special = true;
            }
            if let Ok(true) = get_key(map, "clear_scopes", |x| x.as_bool()) {
                context.clear_scopes = Some(ClearAmount::All);
                is_special = true;
            }
            if let Ok(x) = get_key(map, "clear_scopes", |x| x.as_i64()) {
                context.clear_scopes = Some(ClearAmount::TopN(x as usize));
                is_special = true;
            }
            if !is_special {
                if let Ok(x) = get_key(map, "include", Some) {
                    let reference = SyntaxDefinition::parse_reference(x, state, contexts, namer)?;
                    context.patterns.push(Pattern::Include(reference));
                } else {
                    let pattern =
                        SyntaxDefinition::parse_match_pattern(map, state, contexts, namer)?;
                    if pattern.has_captures {
                        context.uses_backrefs = true;
                    }
                    context.patterns.push(Pattern::Match(pattern));
                }
            }
        }

        contexts.insert(name.clone(), context);
        Ok(name)
    }

    fn parse_reference(
        y: &Yaml,
        state: &mut ParserState<'_>,
        contexts: &mut HashMap<String, Context>,
        namer: &mut ContextNamer,
    ) -> Result<ContextReference, ParseSyntaxError> {
        if let Some(s) = y.as_str() {
            let parts: Vec<&str> = s.split('#').collect();
            let sub_context = if parts.len() > 1 {
                Some(parts[1].to_owned())
            } else {
                None
            };
            if parts[0].starts_with("scope:") {
                Ok(ContextReference::ByScope {
                    scope: state
                        .scope_repo
                        .build(&parts[0][6..])
                        .map_err(ParseSyntaxError::InvalidScope)?,
                    sub_context,
                })
            } else if parts[0].ends_with(".sublime-syntax") {
                let stem = Path::new(parts[0])
                    .file_stem()
                    .and_then(|x| x.to_str())
                    .ok_or(ParseSyntaxError::BadFileRef)?;
                Ok(ContextReference::File {
                    name: stem.to_owned(),
                    sub_context,
                })
            } else {
                Ok(ContextReference::Named(parts[0].to_owned()))
            }
        } else if let Some(v) = y.as_vec() {
            let subname = SyntaxDefinition::parse_context(v, state, contexts, false, namer)?;
            Ok(ContextReference::Inline(subname))
        } else {
            Err(ParseSyntaxError::TypeMismatch)
        }
    }

    fn parse_match_pattern<'l>(
        map: &Hash,
        state: &mut ParserState<'_>,
        contexts: &mut HashMap<String, Context>,
        namer: &mut ContextNamer,
    ) -> Result<MatchPattern<'l>, ParseSyntaxError> {
        let raw_regex = get_key(map, "match", |x| x.as_str())?;
        let regex_str = Self::parse_regex(raw_regex, state)?;
        // println!("{:?}", regex_str);

        let scope = get_key(map, "scope", |x| x.as_str())
            .ok()
            .map(|s| str_to_scopes(s, state.scope_repo))
            .unwrap_or_else(|| Ok(vec![]))?;

        let captures = if let Ok(map) = get_key(map, "captures", |x| x.as_hash()) {
            Some(Self::parse_captures(map, &regex_str, state)?)
        } else {
            None
        };

        let mut has_captures = false;
        let operation = if get_key(map, "pop", Some).is_ok() {
            // Thanks @wbond for letting me know this is the correct way to check for captures
            has_captures = state
                .backref_regex
                .search(&regex_str, 0, regex_str.len(), None);
            MatchOperation::Pop
        } else if let Ok(y) = get_key(map, "push", Some) {
            MatchOperation::Push(SyntaxDefinition::parse_pushargs(y, state, contexts, namer)?)
        } else if let Ok(y) = get_key(map, "set", Some) {
            MatchOperation::Set(SyntaxDefinition::parse_pushargs(y, state, contexts, namer)?)
        } else if let Ok(y) = get_key(map, "embed", Some) {
            // Same as push so we translate it to what it would be
            let mut embed_escape_context_yaml = vec![];
            let mut commands = Hash::new();
            commands.insert(
                Yaml::String("meta_include_prototype".to_string()),
                Yaml::Boolean(false),
            );
            embed_escape_context_yaml.push(Yaml::Hash(commands));
            if let Ok(s) = get_key(map, "embed_scope", Some) {
                commands = Hash::new();
                commands.insert(Yaml::String("meta_content_scope".to_string()), s.clone());
                embed_escape_context_yaml.push(Yaml::Hash(commands));
            }
            if let Ok(v) = get_key(map, "escape", Some) {
                let mut match_map = Hash::new();
                match_map.insert(Yaml::String("match".to_string()), v.clone());
                match_map.insert(Yaml::String("pop".to_string()), Yaml::Boolean(true));
                if let Ok(y) = get_key(map, "escape_captures", Some) {
                    match_map.insert(Yaml::String("captures".to_string()), y.clone());
                }
                embed_escape_context_yaml.push(Yaml::Hash(match_map));
                let escape_context = SyntaxDefinition::parse_context(
                    &embed_escape_context_yaml,
                    state,
                    contexts,
                    false,
                    namer,
                )?;
                MatchOperation::Push(vec![
                    ContextReference::Inline(escape_context),
                    SyntaxDefinition::parse_reference(y, state, contexts, namer)?,
                ])
            } else {
                return Err(ParseSyntaxError::MissingMandatoryKey("escape"));
            }
        } else {
            MatchOperation::None
        };

        let with_prototype = if let Ok(v) = get_key(map, "with_prototype", |x| x.as_vec()) {
            // should a with_prototype include the prototype? I don't think so.
            let subname = Self::parse_context(v, state, contexts, true, namer)?;
            Some(ContextReference::Inline(subname))
        } else if let Ok(v) = get_key(map, "escape", Some) {
            let subname = namer.next();

            let mut context = Context::new(false);
            let mut match_map = Hash::new();
            match_map.insert(
                Yaml::String("match".to_string()),
                Yaml::String(format!("(?={})", v.as_str().unwrap())),
            );
            match_map.insert(Yaml::String("pop".to_string()), Yaml::Boolean(true));
            let pattern =
                SyntaxDefinition::parse_match_pattern(&match_map, state, contexts, namer)?;
            if pattern.has_captures {
                context.uses_backrefs = true;
            }
            context.patterns.push(Pattern::Match(pattern));

            contexts.insert(subname.clone(), context);
            Some(ContextReference::Inline(subname))
        } else {
            None
        };

        let pattern = MatchPattern::new(
            has_captures,
            regex_str,
            scope,
            captures,
            operation,
            with_prototype,
        );

        Ok(pattern)
    }

    fn parse_pushargs(
        y: &Yaml,
        state: &mut ParserState<'_>,
        contexts: &mut HashMap<String, Context>,
        namer: &mut ContextNamer,
    ) -> Result<Vec<ContextReference>, ParseSyntaxError> {
        // check for a push of multiple items
        if y.as_vec().map_or(false, |v| {
            !v.is_empty()
                && (v[0].as_str().is_some()
                    || (v[0].as_vec().is_some() && v[0].as_vec().unwrap()[0].as_hash().is_some()))
        }) {
            // this works because Result implements FromIterator to handle the errors
            y.as_vec()
                .unwrap()
                .iter()
                .map(|x| SyntaxDefinition::parse_reference(x, state, contexts, namer))
                .collect()
        } else {
            let reference = SyntaxDefinition::parse_reference(y, state, contexts, namer)?;
            Ok(vec![reference])
        }
    }

    fn parse_regex(raw_regex: &str, state: &ParserState<'_>) -> Result<String, ParseSyntaxError> {
        let regex = Self::resolve_variables(raw_regex, state);
        let regex = replace_posix_char_classes(regex);
        let regex = if state.lines_include_newline {
            regex_for_newlines(regex)
        } else {
            // If the passed in strings don't include newlines (unlike Sublime) we can't match on
            // them using the original regex. So this tries to rewrite the regex in a way that
            // allows matching against lines without newlines (essentially replacing `\n` with `$`).
            regex_for_no_newlines(regex)
        };
        Self::try_compile_regex(&regex)?;
        Ok(regex)
    }

    fn resolve_variables(raw_regex: &str, state: &ParserState<'_>) -> String {
        let mut result = String::new();
        let mut index = 0;
        let mut region = Region::new();
        while state
            .variable_regex
            .search(raw_regex, index, raw_regex.len(), Some(&mut region))
        {
            let (begin, end) = region.pos(0).unwrap();

            result.push_str(&raw_regex[index..begin]);

            let var_pos = region.pos(1).unwrap();
            let var_name = &raw_regex[var_pos.0..var_pos.1];
            let var_raw = state
                .variables
                .get(var_name)
                .map(String::as_ref)
                .unwrap_or("");
            let var_resolved = Self::resolve_variables(var_raw, state);
            result.push_str(&var_resolved);

            index = end;
        }
        if index < raw_regex.len() {
            result.push_str(&raw_regex[index..]);
        }
        result
    }

    fn try_compile_regex(regex_str: &str) -> Result<(), ParseSyntaxError> {
        // Replace backreferences with a placeholder value that will also appear in errors
        let regex_str =
            substitute_backrefs_in_regex(regex_str, |i| Some(format!("<placeholder_{}>", i)));

        if let Some(error) = Regex::try_compile(&regex_str) {
            Err(ParseSyntaxError::RegexCompileError(regex_str, error))
        } else {
            Ok(())
        }
    }

    fn parse_captures(
        map: &Hash,
        regex_str: &str,
        state: &mut ParserState<'_>,
    ) -> Result<CaptureMapping, ParseSyntaxError> {
        let valid_indexes = get_consuming_capture_indexes(regex_str);
        let mut captures = Vec::new();
        for (key, value) in map.iter() {
            if let (Some(key_int), Some(val_str)) = (key.as_i64(), value.as_str()) {
                if valid_indexes.contains(&(key_int as usize)) {
                    captures.push((key_int as usize, str_to_scopes(val_str, state.scope_repo)?));
                }
            }
        }
        Ok(captures)
    }

    /// Sublime treats the top level context slightly differently from
    /// including the main context from other syntaxes. When main is popped
    /// it is immediately re-added and when it is `set` over the file level
    /// scope remains. This behaviour is emulated through some added contexts
    /// that are the actual top level contexts used in parsing.
    /// See https://github.com/trishume/syntect/issues/58 for more.
    fn add_initial_contexts(
        contexts: &mut HashMap<String, Context>,
        state: &mut ParserState<'_>,
        top_level_scope: Scope,
    ) {
        let yaml_docs = YamlLoader::load_from_str(START_CONTEXT).unwrap();
        let yaml = &yaml_docs[0];

        let start_yaml: &[Yaml] = yaml["__start"].as_vec().unwrap();
        SyntaxDefinition::parse_context(
            start_yaml,
            state,
            contexts,
            false,
            &mut ContextNamer::new("__start"),
        )
        .unwrap();
        if let Some(start) = contexts.get_mut("__start") {
            start.meta_content_scope = vec![top_level_scope];
        }

        let main_yaml: &[Yaml] = yaml["__main"].as_vec().unwrap();
        SyntaxDefinition::parse_context(
            main_yaml,
            state,
            contexts,
            false,
            &mut ContextNamer::new("__main"),
        )
        .unwrap();

        let meta_include_prototype = contexts["main"].meta_include_prototype;
        let meta_scope = contexts["main"].meta_scope.clone();
        let meta_content_scope = contexts["main"].meta_content_scope.clone();

        if let Some(outer_main) = contexts.get_mut("__main") {
            outer_main.meta_include_prototype = meta_include_prototype;
            outer_main.meta_scope = meta_scope;
            outer_main.meta_content_scope = meta_content_scope;
        }

        // add the top_level_scope as a meta_content_scope to main so
        // pushes from other syntaxes add the file scope
        // TODO: this order is not quite correct if main also has a meta_scope
        if let Some(main) = contexts.get_mut("main") {
            main.meta_content_scope.insert(0, top_level_scope);
        }
    }
}
*/
struct ContextNamer {
    name: String,
    anonymous_index: Option<usize>,
}

impl ContextNamer {
    fn new(name: &str) -> ContextNamer {
        ContextNamer {
            name: name.to_string(),
            anonymous_index: None,
        }
    }

    fn next(&mut self) -> String {
        let name = if let Some(index) = self.anonymous_index {
            format!("#anon_{}_{}", self.name, index)
        } else {
            self.name.clone()
        };

        self.anonymous_index = Some(self.anonymous_index.map(|i| i + 1).unwrap_or(0));
        name
    }
}

/// In fancy-regex, POSIX character classes only match ASCII characters.
/// Sublime's syntaxes expect them to match Unicode characters as well, so transform them to
/// corresponding Unicode character classes.
fn replace_posix_char_classes(regex: String) -> String {
    regex
        .replace("[:alpha:]", r"\p{L}")
        .replace("[:alnum:]", r"\p{L}\p{N}")
        .replace("[:lower:]", r"\p{Ll}")
        .replace("[:upper:]", r"\p{Lu}")
        .replace("[:digit:]", r"\p{Nd}")
}

fn regex_for_newlines(regex: String) -> String {
    if !regex.contains('$') {
        return regex;
    }

    let rewriter = RegexRewriterForNewlines {
        parser: Parser::new(regex.as_bytes()),
    };
    rewriter.rewrite()
}

struct RegexRewriterForNewlines<'a> {
    parser: Parser<'a>,
}

impl<'a> RegexRewriterForNewlines<'a> {
    fn rewrite(mut self) -> String {
        let mut result = Vec::new();

        while let Some(c) = self.parser.peek() {
            match c {
                b'$' => {
                    self.parser.next();
                    result.extend_from_slice(br"(?m:$)");
                }
                b'\\' => {
                    self.parser.next();
                    result.push(c);
                    if let Some(c2) = self.parser.peek() {
                        self.parser.next();
                        result.push(c2);
                    }
                }
                b'[' => {
                    let (mut content, _) = self.parser.parse_character_class();
                    result.append(&mut content);
                }
                _ => {
                    self.parser.next();
                    result.push(c);
                }
            }
        }
        String::from_utf8(result).unwrap()
    }
}

/// Rewrite a regex that matches `\n` to one that matches `$` (end of line) instead.
/// That allows the regex to be used to match lines that don't include a trailing newline character.
///
/// The reason we're doing this is because the regexes in the syntax definitions assume that the
/// lines that are being matched on include a trailing newline.
///
/// Note that the rewrite is just an approximation and there's a couple of cases it can not handle,
/// due to `$` being an anchor whereas `\n` matches a character.
fn regex_for_no_newlines(regex: String) -> String {
    if !regex.contains(r"\n") {
        return regex;
    }

    // A special fix to rewrite a pattern from the `Rd` syntax that the RegexRewriter can not
    // handle properly.
    let regex = regex.replace("(?:\\n)?", "(?:$|)");

    let rewriter = RegexRewriterForNoNewlines {
        parser: Parser::new(regex.as_bytes()),
    };
    rewriter.rewrite()
}

struct RegexRewriterForNoNewlines<'a> {
    parser: Parser<'a>,
}

impl<'a> RegexRewriterForNoNewlines<'a> {
    fn rewrite(mut self) -> String {
        let mut result = Vec::new();
        while let Some(c) = self.parser.peek() {
            match c {
                b'\\' => {
                    self.parser.next();
                    if let Some(c2) = self.parser.peek() {
                        self.parser.next();
                        // Replacing `\n` with `$` in `\n?` or `\n+` would make parsing later fail
                        // with "target of repeat operator is invalid"
                        let c3 = self.parser.peek();
                        if c2 == b'n' && c3 != Some(b'?') && c3 != Some(b'+') && c3 != Some(b'*') {
                            result.extend_from_slice(b"$");
                        } else {
                            result.push(c);
                            result.push(c2);
                        }
                    } else {
                        result.push(c);
                    }
                }
                b'[' => {
                    let (mut content, matches_newline) = self.parser.parse_character_class();
                    if matches_newline && self.parser.peek() != Some(b'?') {
                        result.extend_from_slice(b"(?:");
                        result.append(&mut content);
                        result.extend_from_slice(br"|$)");
                    } else {
                        result.append(&mut content);
                    }
                }
                _ => {
                    self.parser.next();
                    result.push(c);
                }
            }
        }
        String::from_utf8(result).unwrap()
    }
}

fn get_consuming_capture_indexes(regex: &str) -> Vec<usize> {
    let parser = ConsumingCaptureIndexParser {
        parser: Parser::new(regex.as_bytes()),
    };
    parser.get_consuming_capture_indexes()
}

struct ConsumingCaptureIndexParser<'a> {
    parser: Parser<'a>,
}

impl<'a> ConsumingCaptureIndexParser<'a> {
    /// find capture groups which are not inside lookarounds.
    /// If, in a YAML syntax definition, a scope stack is applied
    /// to a capture group inside a lookaround,
    /// (i.e. "captures:\n x: scope.stack goes.here", where "x" is
    /// the number of a capture group in a lookahead/behind), those
    /// those scopes are not applied, so no need to even parse them.
    fn get_consuming_capture_indexes(mut self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut cap_num = 0;
        let mut in_lookaround = false;
        stack.push(in_lookaround);
        result.push(cap_num);

        while let Some(c) = self.parser.peek() {
            match c {
                b'\\' => {
                    self.parser.next();
                    self.parser.next();
                }
                b'[' => {
                    self.parser.parse_character_class();
                }
                b'(' => {
                    self.parser.next();
                    // add the current lookaround state to the stack so we can just pop at a closing paren
                    stack.push(in_lookaround);
                    if let Some(c2) = self.parser.peek() {
                        if c2 != b'?' {
                            // simple numbered capture group
                            cap_num += 1;
                            // if we are not currently in a lookaround,
                            // add this capture group number to the valid ones
                            if !in_lookaround {
                                result.push(cap_num);
                            }
                        } else {
                            self.parser.next();
                            if let Some(c3) = self.parser.peek() {
                                self.parser.next();
                                if c3 == b'=' || c3 == b'!' {
                                    // lookahead
                                    in_lookaround = true;
                                } else if c3 == b'<' {
                                    if let Some(c4) = self.parser.peek() {
                                        if c4 == b'=' || c4 == b'!' {
                                            self.parser.next();
                                            // lookbehind
                                            in_lookaround = true;
                                        }
                                    }
                                } else if c3 == b'P' {
                                    if let Some(c4) = self.parser.peek() {
                                        if c4 == b'<' {
                                            // named capture group
                                            cap_num += 1;
                                            // if we are not currently in a lookaround,
                                            // add this capture group number to the valid ones
                                            if !in_lookaround {
                                                result.push(cap_num);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                b')' => {
                    if let Some(value) = stack.pop() {
                        in_lookaround = value;
                    }
                    self.parser.next();
                }
                _ => {
                    self.parser.next();
                }
            }
        }
        result
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(bytes: &[u8]) -> Parser {
        Parser { bytes, index: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).map(|&b| b)
    }

    fn next(&mut self) {
        self.index += 1;
    }

    fn parse_character_class(&mut self) -> (Vec<u8>, bool) {
        let mut content = Vec::new();
        let mut negated = false;
        let mut nesting = 0;
        let mut matches_newline = false;

        self.next();
        content.push(b'[');
        if let Some(b'^') = self.peek() {
            self.next();
            content.push(b'^');
            negated = true;
        }

        // An unescaped `]` is allowed after `[` or `[^` and doesn't mean the end of the class.
        if let Some(b']') = self.peek() {
            self.next();
            content.push(b']');
        }

        while let Some(c) = self.peek() {
            match c {
                b'\\' => {
                    self.next();
                    content.push(c);
                    if let Some(c2) = self.peek() {
                        self.next();
                        if c2 == b'n' && !negated && nesting == 0 {
                            matches_newline = true;
                        }
                        content.push(c2);
                    }
                }
                b'[' => {
                    self.next();
                    content.push(b'[');
                    nesting += 1;
                }
                b']' => {
                    self.next();
                    content.push(b']');
                    if nesting == 0 {
                        break;
                    }
                    nesting -= 1;
                }
                _ => {
                    self.next();
                    content.push(c);
                }
            }
        }

        (content, matches_newline)
    }
}
