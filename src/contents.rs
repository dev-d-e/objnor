use crate::parser::{Parser, END};
use crate::tree::{Node, Root};
use crate::xml::add_tag;
use std::collections::HashMap;

///Trait that is represents action when parse text.
pub trait Contents {
    ///Result type.
    type Item;

    ///Get result.
    fn get(self) -> Self::Item;

    ///Accept offset and prepare to accept key.
    fn pre_key(&mut self, offset: usize);

    ///Accept char for key.
    fn key(&mut self, c: char);

    ///End of key.
    fn post_key(&mut self);

    ///Prepare to accept text.
    fn pre_text(&mut self);

    ///Accept char for text.
    fn text(&mut self, c: char);

    ///Prepare to accept next text in array.
    fn text_array(&mut self);

    ///End of text.
    fn post_text(&mut self);

    ///Wrong char in the row and number.
    fn error(&mut self, row: usize, n: usize);
}

///Result object.
#[derive(Clone, Debug)]
pub struct Target {
    pub name: String,
    pub text: Vec<String>,
    pub value: Vec<Target>,
}

impl Target {
    pub fn new(name: String) -> Self {
        Target {
            name,
            text: Vec::new(),
            value: Vec::new(),
        }
    }
}

struct Builder<T: FnMut(usize, usize)> {
    tree: Root,
    offset: usize,
    key: String,
    text: String,
    err_function: T,
}

impl<T: FnMut(usize, usize)> Contents for Builder<T> {
    type Item = Vec<Target>;

    fn get(self) -> Self::Item {
        self.tree.build()
    }

    fn pre_key(&mut self, offset: usize) {
        self.offset = offset;
        self.key.clear();
    }

    fn key(&mut self, c: char) {
        self.key.push(c);
    }

    fn post_key(&mut self) {}

    fn pre_text(&mut self) {
        self.text.clear();
    }

    fn text(&mut self, c: char) {
        self.text.push(c);
    }

    fn text_array(&mut self) {
        self.post_text();
        self.pre_text();
    }

    fn post_text(&mut self) {
        let n = Node::new(self.offset, self.key.clone(), self.text.clone());
        self.tree.add(n);
        self.text.clear();
    }

    fn error(&mut self, row: usize, n: usize) {
        (self.err_function)(row, n);
    }
}

impl<T: FnMut(usize, usize)> Builder<T> {
    fn new(func: T) -> Self {
        Builder {
            tree: Root::new(),
            offset: 0,
            key: String::new(),
            text: String::new(),
            err_function: func,
        }
    }
}

///Parse text format.
pub fn parse_chars<T, S, R>(mut iter: S, contents: R) -> T
where
    S: Iterator<Item = char>,
    R: Contents<Item = T>,
{
    let mut parser = Parser::new(contents);
    while let Some(c) = iter.next() {
        parser.accept(c);
    }
    parser.accept(END);
    parser.contents().get()
}

///Parse text format.
pub fn parse_slice<T, S>(buf: &[char], contents: S) -> T
where
    S: Contents<Item = T>,
{
    let mut parser = Parser::new(contents);
    for c in buf {
        parser.accept(*c);
    }
    parser.accept(END);
    parser.contents().get()
}

///Parse text format to `Vec<Target>`.
pub fn chars_to_target<T>(iter: T, func: impl FnMut(usize, usize)) -> Vec<Target>
where
    T: Iterator<Item = char>,
{
    parse_chars(iter, Builder::new(func))
}

///Parse text format to `Vec<Target>`.
pub fn slice_to_target(buf: &[char], func: impl FnMut(usize, usize)) -> Vec<Target> {
    parse_slice(buf, Builder::new(func))
}

///Convert text format `Vec<Target>` to `HashMap`. use separator to join key.
pub fn vec_to_map(vec: Vec<Target>, separator: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for target in vec {
        let mut key_vec = Vec::new();
        target_to_map(target, &mut key_vec, &mut map);
    }
    map.into_iter()
        .map(|(k, v)| (k.join(separator), v))
        .collect()
}

///Convert text format `Target` to `HashMap`. use separator to join key.
pub fn to_map(target: Target, separator: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let mut key_vec = Vec::new();
    target_to_map(target, &mut key_vec, &mut map);
    map.into_iter()
        .map(|(k, v)| (k.join(separator), v))
        .collect()
}

fn target_to_map(
    target: Target,
    key_vec: &mut Vec<String>,
    map: &mut HashMap<Vec<String>, Vec<String>>,
) {
    key_vec.push(target.name);
    if target.text.len() > 0 {
        map.insert(key_vec.to_vec(), target.text);
    }
    for c in target.value {
        target_to_map(c, key_vec, map);
    }
    key_vec.pop();
}

///Convert text format `Vec<Target>` to xml format.
pub fn vec_to_xml(vec: Vec<Target>) -> String {
    let mut xml = String::new();
    for target in vec {
        xml.push_str(&to_xml(target));
    }
    xml
}

///Convert text format `Target` to xml format.
pub fn to_xml(target: Target) -> String {
    let mut xml = String::new();
    let n = &target.name;
    for t in target.text {
        add_tag(&mut xml, n, &t);
    }
    let s = vec_to_xml(target.value);
    if s.len() > 0 {
        add_tag(&mut xml, n, &s);
    }
    xml
}
