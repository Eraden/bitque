pub mod wasm {
    use std::error::Error;

    use super::regex_impl;
    use serde::Deserializer;
    use std::borrow::Cow;

    pub struct Regex {
        pub regex: regex_impl::Regex,
        pub regex_str: Cow<'static, str>,
    }

    impl<'de> serde::Deserialize<'de> for Regex {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let regex_str = String::deserialize(deserializer)?;
            Ok(Regex::new_with_owned(regex_str))
        }
    }

    impl Regex {
        pub fn new(regex_str: &'static str) -> Self {
            let regex =
                regex_impl::Regex::new(regex_str).expect("regex string should be pre-tested");

            Self {
                regex,
                regex_str: Cow::Borrowed(regex_str),
            }
        }
        pub fn new_with_owned(regex_str: String) -> Self {
            let regex = regex_impl::Regex::new(regex_str.as_str())
                .expect("regex string should be pre-tested");

            Self {
                regex,
                regex_str: Cow::Owned(regex_str),
            }
        }

        pub fn regex_str(&self) -> &str {
            match self.regex_str {
                Cow::Borrowed(s) => s,
                Cow::Owned(ref o) => o,
            }
        }

        /// Check whether the pattern compiles as a valid regex or not.
        pub fn try_compile(regex_str: &str) -> Option<Box<dyn Error + Send>> {
            regex_impl::Regex::new(regex_str).err()
        }

        pub fn is_match(&self, text: &str) -> bool {
            self.regex().is_match(text)
        }

        pub fn search(
            &self,
            text: &str,
            begin: usize,
            end: usize,
            region: Option<&mut Region>,
        ) -> bool {
            self.regex()
                .search(text, begin, end, region.map(|r| &mut r.region))
        }

        fn regex(&self) -> &regex_impl::Regex {
            &self.regex
        }
    }

    impl Region {
        pub fn new() -> Self {
            Self {
                region: regex_impl::new_region(),
            }
        }

        /// Get the start/end positions of the capture group with given index.
        ///
        /// If there is no match for that group or the index does not correspond to a group, `None` is
        /// returned. The index 0 returns the whole match.
        pub fn pos(&self, index: usize) -> Option<(usize, usize)> {
            self.region.pos(index)
        }
    }

    #[derive(Eq, PartialEq, Clone)]
    pub struct Region {
        pub region: regex_impl::Region,
    }
}

#[cfg(feature = "wasm")]
pub use wasm::*;

mod regex_impl {
    use std::error::Error;

    #[derive(Debug)]
    pub struct Regex {
        pub regex: fancy_regex::Regex,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Region {
        positions: Vec<Option<(usize, usize)>>,
    }

    pub fn new_region() -> Region {
        Region {
            positions: Vec::with_capacity(8),
        }
    }

    impl Regex {
        pub fn new(regex_str: &str) -> Result<Regex, Box<dyn Error + Send>> {
            let result = fancy_regex::Regex::new(regex_str);
            match result {
                Ok(regex) => Ok(Regex { regex }),
                Err(error) => Err(Box::new(error)),
            }
        }

        pub fn is_match(&self, text: &str) -> bool {
            self.regex.is_match(text).unwrap_or(false)
        }

        pub fn search(
            &self,
            text: &str,
            begin: usize,
            end: usize,
            region: Option<&mut Region>,
        ) -> bool {
            // If there's an error during search, treat it as non-matching.
            // For example, in case of catastrophic backtracking, fancy-regex should
            // fail with an error eventually.
            if let Ok(Some(captures)) = self.regex.captures_from_pos(&text[..end], begin) {
                if let Some(region) = region {
                    region.init_from_captures(&captures);
                }
                true
            } else {
                false
            }
        }
    }

    impl Region {
        fn init_from_captures(&mut self, captures: &fancy_regex::Captures) {
            self.positions.clear();
            for i in 0..captures.len() {
                let pos = captures.get(i).map(|m| (m.start(), m.end()));
                self.positions.push(pos);
            }
        }

        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            if i < self.positions.len() {
                self.positions[i]
            } else {
                None
            }
        }
    }
}
