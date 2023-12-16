use crate::parser::{Parser, END};
use crate::tree::{Node, Root};

pub trait Contents {
    type Item;

    fn get(self) -> Self::Item;

    fn pre_key(&mut self, offset: usize);

    fn key(&mut self, c: char);

    fn post_key(&mut self);

    fn pre_text(&mut self);

    fn text(&mut self, c: char);

    fn text_array(&mut self);

    fn post_text(&mut self);

    fn error(&mut self, row: usize, n: usize);
}

#[derive(Debug)]
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

struct Builder {
    tree: Root,
    offset: usize,
    key: String,
    text: String,
    err_function: fn(usize, usize),
}

impl Contents for Builder {
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

impl Builder {
    fn new(func: fn(usize, usize)) -> Self {
        Builder {
            tree: Root::new(),
            offset: 0,
            key: String::new(),
            text: String::new(),
            err_function: func,
        }
    }
}

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

pub fn parse_chars_to_target<T>(iter: T, func: fn(usize, usize)) -> Vec<Target>
where
    T: Iterator<Item = char>,
{
    parse_chars(iter, Builder::new(func))
}

pub fn parse_slice_to_target(buf: &[char], func: fn(usize, usize)) -> Vec<Target> {
    parse_slice(buf, Builder::new(func))
}
