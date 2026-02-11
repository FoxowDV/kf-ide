#![allow(dead_code)]
pub mod kf;
pub mod config_theme; 
//
//use std::collections::BTreeSet;
//use std::hash::{
//    Hash,
//    Hasher
//};
//
//
//pub const SEPARATORS: [char; 1] = ['_'];
//pub const QUOTES: [char; 3] = ['\'', '"', '`'];
//
//type MultiLine = bool;
//type Float = bool;
//
//#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
//pub enum TokenType {
//    Comment(MultiLine),
//    Function,
//    Keyword,
//    Literal,
//    Hyperlink,
//    Numeric(Float),
//    Punctuation(char),
//    Special,
//    Str(char),
//    Type,
//    Whitespace(char),
//    #[default]
//    Unknown,
//}
//
//
//#[derive(Clone, Debug, PartialEq)]
//pub struct Syntax {
//    pub language: &'static str,
//    pub case_sensitive: bool,
//    pub comment: &'static str,
//    pub comment_multiline: [&'static str; 2],
//    pub hyperlinks: BTreeSet<&'static str>,
//    pub keywords: BTreeSet<&'static str>,
//    pub types: BTreeSet<&'static str>,
//    pub special: BTreeSet<&'static str>,
//}
//
//impl Default for Syntax {
//    fn default() -> Self {
//        Syntax::kf()
//    }
//}
//
//impl Hash for Syntax {
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        self.language.hash(state);
//    }
//}
//
