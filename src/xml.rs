fn start_tag(s: &mut String, tag_name: &str) {
    s.push('<');
    s.push_str(tag_name);
    s.push('>');
}

fn start_tag_attribute(s: &mut String, tag_name: &str, attribute: &str) {
    s.push('<');
    s.push_str(tag_name);
    if attribute.len() > 0 {
        if !attribute.starts_with(' ') {
            s.push(' ');
        }
        s.push_str(attribute);
    }
    s.push('>');
}

fn end_tag(s: &mut String, tag_name: &str) {
    s.push('<');
    s.push('/');
    s.push_str(tag_name);
    s.push('>');
}

//add xml format attribute
pub(crate) fn add_attribute(s: &mut String, attribute_name: &str, attribute_value: &[String]) {
    s.push(' ');
    s.push_str(attribute_name);
    s.push('=');
    s.push('\"');
    let mut value = String::new();
    for v in attribute_value {
        value.push_str(&v);
        value.push(' ');
    }
    value.pop();
    s.push_str(&value);
    s.push('\"');
}

//add xml format tag
pub(crate) fn add_tag(s: &mut String, tag_name: &str, text: &str) {
    start_tag(s, tag_name);
    if text.len() > 0 {
        s.push_str(text);
    }
    end_tag(s, tag_name);
}

//add xml format tag with attribute
pub(crate) fn add_tag_with_attribute(s: &mut String, tag_name: &str, attribute: &str, text: &str) {
    start_tag_attribute(s, tag_name, attribute);
    if text.len() > 0 {
        s.push_str(text);
    }
    end_tag(s, tag_name);
}
