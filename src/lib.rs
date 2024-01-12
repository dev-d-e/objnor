//!A text format for object notation.
//!
//!text.
//!
//!```
//!0~a:a
//!+a
//!|b
//!|c
//!0~b:a
//!1~a
//!2~a:b
//!0~c:
//!|
//!|cc
//!|
//!```
//!
//!object.
//!
//!```
//![Target { name: "a", text: ["a", "a\nb\nc"], value: [] },
//!
//!Target { name: "b", text: ["a"], value: [Target { name: "a", text: [], value: [Target { name: "a", text: ["b"], value: [] }] }] },
//!
//!Target { name: "c", text: ["\n\ncc\n"], value: [] }]
//!```
//!
//!* start with offset and tilde(~), then data key, then colon(:). If next offset is greater, it's child object.
//!
//!* multiple lines text use vertical(|).
//!
//!* array text use plus(+).
//!

#![allow(dead_code)]

mod contents;
mod header;
///To HTML String.
pub mod html;
mod parser;
mod tree;
mod xml;

pub use crate::contents::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let s = "0~a:a
+a
|b
|c
0~b:a
1~a
2~a:b
0~c:
|
|cc
|
";
        let t = parse_chars_to_target(s.chars(), |r, n| println!("err ({},{})", r, n));
        println!("{:?}", t);
        assert_eq!(t[1].text[0], "a");
        assert_eq!(t[1].value[0].value[0].text[0], "b");
    }
}
