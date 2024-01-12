use crate::contents::Contents;
use crate::header::Header;

//'\r'
const CR: char = '\r';
//'\n'
const LF: char = '\n';
//'#'
const NUMBERSIGN: char = '#';
//'~'
const TILDE: char = '~';
//':'
const COLON: char = ':';
//'|'
const VERTICAL: char = '|';
//'+'
const PLUS: char = '+';
//' '
const SPACE: char = ' ';
//'\'
const BACKSLASH: char = '\\';
//0
pub(crate) const END: char = 0 as char;

fn is_crlf(c: char) -> bool {
    c == CR || c == LF
}

pub(crate) struct Parser<T: Contents> {
    n: usize,
    row: usize,
    current_function: fn(&mut Parser<T>, char),
    new_line: NewLine,
    header_parser: HeaderParser,
    offset: Offset,
    contents: T,
}

impl<T: Contents> Parser<T> {
    pub(crate) fn new(contents: T) -> Self {
        Parser {
            n: 0,
            row: 1,
            current_function: Self::accept_pre_header,
            new_line: NewLine::new(),
            header_parser: HeaderParser::new(),
            offset: Offset::new(),
            contents,
        }
    }

    fn reset(&mut self) {
        self.n = 0;
        self.row = 1;
        self.current_function = Self::accept_pre_header;
        self.new_line = NewLine::new();
        self.header_parser = HeaderParser::new();
        self.offset = Offset::new();
    }

    pub(crate) fn contents(self) -> T {
        self.contents
    }

    pub(crate) fn accept(&mut self, c: char) {
        self.n += 1;
        (self.current_function)(self, c);
    }

    fn accept_pre_header(&mut self, c: char) {
        self.header_parser.pre();
        if c == NUMBERSIGN {
            self.current_function = Self::accept_header;
            return;
        } else if is_crlf(c) {
            self.error();
            return;
        }
        self.current_function = Self::accept_pre_offset;
        self.accept_pre_offset(c);
    }

    fn accept_header(&mut self, c: char) {
        if c == NUMBERSIGN {
            return;
        } else if is_crlf(c) {
            if !self.header_parser.post() {
                self.error();
            }
            self.current_function = Self::header_new_line;
            self.header_new_line(c);
            return;
        }
        self.header_parser.accept(c);
    }

    fn header_new_line(&mut self, c: char) {
        let s = self.new_line.accept(c);
        if s.len() > 0 {
            self.row += 1;
            self.n = 0;
        }
        if is_crlf(c) {
            return;
        }
        self.current_function = Self::accept_pre_header;
        self.accept_pre_header(c);
    }

    fn accept_pre_offset(&mut self, c: char) {
        self.offset.pre();
        if c.is_ascii_hexdigit() {
            self.current_function = Self::accept_offset;
            self.accept_offset(c);
            return;
        } else if is_crlf(c) {
            self.current_function = Self::offset_new_line;
            self.offset_new_line(c);
            return;
        } else if c == END {
            self.reset();
            return;
        }
        self.error();
        self.current_function = Self::offset_error;
    }

    fn accept_offset(&mut self, c: char) {
        if c.is_ascii_hexdigit() {
            self.offset.accept(c);
            return;
        } else if c == TILDE {
            if self.offset.post() {
                self.current_function = Self::accept_pre_key;
                return;
            }
        } else if is_crlf(c) {
            self.error();
            self.current_function = Self::offset_new_line;
            self.offset_new_line(c);
            return;
        } else if c == END {
            self.reset();
            return;
        }
        self.error();
        self.current_function = Self::offset_error;
    }

    fn offset_error(&mut self, c: char) {
        if is_crlf(c) {
            self.current_function = Self::offset_new_line;
            self.offset_new_line(c);
        }
    }

    fn offset_new_line(&mut self, c: char) {
        let s = self.new_line.accept(c);
        if s.len() > 0 {
            self.row += 1;
            self.n = 0;
        }
        if is_crlf(c) {
            return;
        }
        self.current_function = Self::accept_pre_offset;
        self.accept_pre_offset(c);
    }

    fn accept_pre_key(&mut self, c: char) {
        self.contents.pre_key(self.offset.number);
        if c == COLON {
            self.error();
            self.current_function = Self::key_error;
            return;
        } else if is_crlf(c) {
            self.error();
            self.current_function = Self::offset_new_line;
            self.offset_new_line(c);
            return;
        }
        self.current_function = Self::accept_key;
        self.accept_key(c);
    }

    fn accept_key(&mut self, c: char) {
        if c == COLON {
            self.contents.post_key();
            self.current_function = Self::accept_pre_text;
            return;
        } else if c == BACKSLASH {
            self.current_function = Self::key_backslash;
            return;
        } else if is_crlf(c) {
            self.contents.post_key();
            self.contents.post_text();
            self.current_function = Self::offset_new_line;
            self.offset_new_line(c);
            return;
        } else if c == END {
            self.contents.post_key();
            self.contents.post_text();
            self.reset();
            return;
        }
        self.contents.key(c);
    }

    fn key_backslash(&mut self, c: char) {
        if is_crlf(c) {
            self.current_function = Self::accept_key;
            self.accept_key(c);
            return;
        }
        self.contents.key(c);
        self.current_function = Self::accept_key;
    }

    fn key_error(&mut self, c: char) {
        if is_crlf(c) {
            self.current_function = Self::accept_pre_offset;
            self.accept_pre_offset(c);
        }
    }

    fn accept_pre_text(&mut self, c: char) {
        self.contents.pre_text();
        if is_crlf(c) {
            self.current_function = Self::text_new_line;
            self.text_new_line(c);
            return;
        }
        self.current_function = Self::accept_text;
        self.accept_text(c);
    }

    fn accept_text(&mut self, c: char) {
        if is_crlf(c) {
            self.current_function = Self::text_new_line;
            self.text_new_line(c);
            return;
        } else if c == END {
            self.text_new_line(c);
            return;
        }
        self.contents.text(c);
    }

    fn text_new_line(&mut self, c: char) {
        if c == SPACE {
            return;
        }
        let s = self.new_line.accept(c);
        if s.len() > 0 {
            self.row += 1;
            self.n = 0;
        } else if is_crlf(c) {
            return;
        }
        if s.len() > 0 && c == VERTICAL {
            for i in s {
                self.contents.text(*i);
            }
        }
        if c == VERTICAL || c == PLUS {
            self.current_function = Self::text_more;
            self.text_more(c);
            return;
        }
        self.contents.post_text();
        if c == END {
            self.reset();
            return;
        }
        self.current_function = Self::accept_pre_offset;
        self.accept_pre_offset(c);
    }

    fn text_more(&mut self, c: char) {
        if c == VERTICAL {
            self.current_function = Self::accept_text;
            return;
        } else if c == PLUS {
            self.contents.text_array();
            self.current_function = Self::accept_text;
            return;
        }
        self.error();
        if is_crlf(c) {
            self.current_function = Self::text_new_line;
            self.text_new_line(c);
            return;
        }
        self.current_function = Self::text_more_error;
    }

    fn text_more_error(&mut self, c: char) {
        if is_crlf(c) {
            self.current_function = Self::text_new_line;
            self.text_new_line(c);
        }
    }

    fn error(&mut self) {
        self.contents.error(self.row, self.n);
    }
}

struct NewLine {
    has_cr: bool,
    has_lf: bool,
    cr_lf: Vec<char>,
}

impl NewLine {
    fn new() -> Self {
        NewLine {
            has_cr: false,
            has_lf: false,
            cr_lf: Vec::new(),
        }
    }

    fn accept(&mut self, c: char) -> &Vec<char> {
        self.cr_lf.clear();
        if self.has_cr && self.has_lf {
            self.cr_lf.push(CR);
            self.cr_lf.push(LF);
            if c == CR {
                self.has_lf = false;
            } else if c == LF {
                self.has_cr = false;
            } else {
                self.has_cr = false;
                self.has_lf = false;
            }
        } else if self.has_cr {
            if c == CR {
                self.cr_lf.push(CR);
            } else if c == LF {
                self.has_lf = true;
            } else {
                self.has_cr = false;
                self.cr_lf.push(CR);
            }
        } else if self.has_lf {
            if c == CR {
                self.has_cr = true;
                self.has_lf = false;
                self.cr_lf.push(LF);
            } else if c == LF {
                self.cr_lf.push(LF);
            } else {
                self.has_lf = false;
                self.cr_lf.push(LF);
            }
        } else if c == CR {
            self.has_cr = true;
        } else if c == LF {
            self.has_lf = true;
        }
        &self.cr_lf
    }
}

struct HeaderParser {
    header: Header,
    str: String,
}

impl HeaderParser {
    fn new() -> Self {
        HeaderParser {
            header: Header::new(),
            str: String::new(),
        }
    }

    fn pre(&mut self) {
        self.str.clear();
    }

    fn accept(&mut self, c: char) {
        self.str.push(c);
    }

    fn post(&mut self) -> bool {
        self.header.accept(self.str.clone());
        self.str.clear();
        true
    }
}

struct Offset {
    available: usize,
    number: usize,
    str: String,
}

impl Offset {
    fn new() -> Self {
        Offset {
            available: 0,
            number: 0,
            str: String::new(),
        }
    }

    fn pre(&mut self) {
        self.str.clear();
    }

    fn accept(&mut self, c: char) {
        self.str.push(c);
    }

    fn post(&mut self) -> bool {
        match usize::from_str_radix(&self.str, 16) {
            Ok(n) => {
                self.number = n;
            }
            Err(e) => {
                println!("{:?}", e);
                return false;
            }
        }
        if self.number > self.available {
            return false;
        }
        self.available = self.number + 1;
        true
    }
}
