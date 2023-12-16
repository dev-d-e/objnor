pub(crate) struct Header {
    v: Vec<String>,
}

impl Header {
    pub(crate) fn new() -> Self {
        Header { v: Vec::new() }
    }

    pub(crate) fn accept(&mut self, str: String) {
        self.v.push(str);
    }
}
