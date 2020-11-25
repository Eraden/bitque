pub mod native {
    use std::error::Error;

    use super::regex_impl;
    use lazycell::AtomicLazyCell;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub struct Regex {
        pub regex_str: String,
        pub regex: AtomicLazyCell<regex_impl::Regex>,
    }

    impl Clone for Regex {
        fn clone(&self) -> Self {
            Self {
                regex_str: self.regex_str.clone(),
                regex: AtomicLazyCell::new(),
            }
        }
    }

    impl std::fmt::Debug for Regex {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(format!("Regex::new({:#?})", self.regex_str).as_str())
        }
    }

    impl Regex {
        pub fn new<S: Into<String>>(regex_str: S) -> Self {
            Self {
                regex_str: regex_str.into(),
                regex: AtomicLazyCell::new(),
            }
        }

        /// Check whether the pattern compiles as a valid regex or not.
        pub fn try_compile(regex_str: &str) -> Option<Box<dyn Error + Send>> {
            regex_impl::Regex::new(regex_str).err()
        }

        /// Return the regex pattern.
        pub fn regex_str(&self) -> &str {
            &self.regex_str
        }

        /// Check if the regex matches the given text.
        pub fn is_match(&self, text: &str) -> bool {
            self.regex().is_match(text)
        }

        /// Search for the pattern in the given text from begin/end positions.
        ///
        /// If a region is passed, it is used for storing match group positions. The argument allows
        /// the `Region` to be reused between searches, which makes a significant performance
        /// difference.
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
            if let Some(regex) = self.regex.borrow() {
                regex
            } else {
                let regex = regex_impl::Regex::new(&self.regex_str)
                    .expect("regex string should be pre-tested");
                self.regex.fill(regex).ok();
                self.regex.borrow().unwrap()
            }
        }
    }

    impl PartialEq for Regex {
        fn eq(&self, other: &Regex) -> bool {
            self.regex_str == other.regex_str
        }
    }

    impl Eq for Regex {}

    impl Serialize for Regex {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.regex_str)
        }
    }

    impl<'de> Deserialize<'de> for Regex {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let regex_str = String::deserialize(deserializer)?;
            Ok(Regex::new(regex_str))
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

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Region {
        pub region: regex_impl::Region,
    }
}

pub use native::*;

mod regex_impl {
    use std::error::Error;

    #[derive(Debug)]
    pub struct Regex {
        regex: fancy_regex::Regex,
    }

    impl Clone for Regex {
        fn clone(&self) -> Self {
            unreachable!()
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Region {
        positions: Vec<Option<(usize, usize)>>,
    }

    impl std::fmt::Debug for Region {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("regex_impl::Region {\n")?;
            f.write_str(format!("positions: vec!{:#?},\n", self.positions).as_str())?;
            f.write_str("}")
        }
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
            // Errors are treated as non-matches
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
