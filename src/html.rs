use crate::xml::{add_attribute, add_tag, add_tag_with_attribute};
use crate::Target;
use std::collections::{HashMap, HashSet};

const GLOBALS: [&str; 25] = [
    "class",
    "id",
    "slot",
    "accesskey",
    "autocapitalize",
    "autofocus",
    "contenteditable",
    "dir",
    "draggable",
    "enterkeyhint",
    "hidden",
    "inputmode",
    "is",
    "itemid",
    "itemprop",
    "itemref",
    "itemscope",
    "itemtype",
    "lang",
    "nonce",
    "spellcheck",
    "style",
    "tabindex",
    "title",
    "translate",
];

const EVENT_HANDLER: [&str; 66] = [
    "onabort",
    "onauxclick",
    "onblur",
    "oncancel",
    "oncanplay",
    "oncanplaythrough",
    "onchange",
    "onclick",
    "onclose",
    "oncontextmenu",
    "oncopy",
    "oncuechange",
    "oncut",
    "ondblclick",
    "ondrag",
    "ondragend",
    "ondragenter",
    "ondragexit",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "ondurationchange",
    "onemptied",
    "onended",
    "onerror",
    "onfocus",
    "onformdata",
    "oninput",
    "oninvalid",
    "onkeydown",
    "onkeypress",
    "onkeyup",
    "onload",
    "onloadeddata",
    "onloadedmetadata",
    "onloadstart",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onmouseup",
    "onpaste",
    "onpause",
    "onplay",
    "onplaying",
    "onprogress",
    "onratechange",
    "onreset",
    "onresize",
    "onscroll",
    "onsecuritypolicyviolation",
    "onseeked",
    "onseeking",
    "onselect",
    "onslotchange",
    "onstalled",
    "onsubmit",
    "onsuspend",
    "ontimeupdate",
    "ontoggle",
    "onvolumechange",
    "onwaiting",
    "onwheel",
];

const DATA: &str = "data-";

const ELEMENT_ATTRIBUTES: [(&str, &str); 47] = [
    ("a","href,target,download,ping,rel,hreflang,type,referrerpolicy"),
    ("area","alt,coords,shape,href,target,download,ping,rel,referrerpolicy"),
    ("audio","src,crossorigin,preload,autoplay,loop,muted,controls"),
    ("base", "href,target"),
    ("blockquote", "cite"),
    ("body","onafterprint,onbeforeprint,onbeforeunload,onhashchange,onlanguagechange,onmessage,onmessageerror,onoffline,ononline,onpagehide,onpageshow,onpopstate,onrejectionhandled,onstorage,onunhandledrejection,onunload"),
    ("button", "disabled,form,formaction,formenctype,formmethod,formnovalidate,formtarget,name,type,value"),
    ("canvas", "width,height"),
    ("col", "span"),
    ("colgroup", "span"),
    ("data", "value"),
    ("del", "cite,datetime"),
    ("details", "open"),
    ("dialog", "open"),
    ("embed", "src,type,width,height"),
    ("fieldset", "disabled,form,name"),
    ("form", "accept-charset,action,autocomplete,enctype,method,name,novalidate,target"),
    ("html", "manifest"),
    ("iframe", "src,srcdoc,name,sandbox,allow,allowfullscreen,allowpaymentrequest,width,height,referrerpolicy,loading"),
    ("img", "alt,src,srcset,sizes,crossorigin,usemap,ismap,width,height,referrerpolicy,decoding,loading"),
    ("input", "accept,alt,autocomplete,checked,dirname,disabled,form,formaction,formenctype,formmethod ,formnovalidate,formtarget,height,list,max,maxlength ,min,minlength,multiple,name,pattern,placeholder,readonly,required,size,src,step,type,value,width"),
    ("ins", "cite,datetime"),
    ("label", "for"),
    ("li", "value"),
    ("link", "href,crossorigin,rel,as,media,hreflang,type,sizes,imagesrcset,imagesizes,referrerpolicy,integrity,color,disabled"),
    ("map", "name"),
    ("meta", "name,http-equiv,content,charset"),
    ("meter", "value,min,max,low,high,optimum"),
    ("object", "data,type,name,usemap,form,width,height"),
    ("ol", "reversed,start,type"),
    ("optgroup","disabled,label"),
    ("option", "disabled,label,selected,value"),
    ("output", "for,form,name"),
    ("param", "name,value"),
    ("progress", "value,max"),
    ("q", "cite"),
    ("script", "src,type,async,defer,crossorigin,integrity,referrerpolicy"),
    ("select", "autocomplete,disabled,form,multiple,name,required,size"),
    ("slot", "name"),
    ("source", "src,type,srcset,sizes,media"),
    ("style", "media"),
    ("td", "colspan,rowspan,headers"),
    ("textarea", "cols,dirname,disabled,form,maxlength,minlength,name,placeholder,readonly,required,rows,wrap"),
    ("th","colspan,rowspan,headers,scope,abbr"),
    ("time", "datetime"),
    ("track", "default,kind,label,src,srclang"),
    ("video", "src,crossorigin,poster,preload,autoplay,playsinline,loop,muted,controls,width,height"),
];

//special case
fn is_element(element: &str, s: &str) -> bool {
    if element == "head" && s == "title" {
        return true;
    }
    false
}

///Convert `Target` to HTML String.
pub struct Converter {
    globals: HashSet<&'static str>,
    event_handler: HashSet<&'static str>,
    element_attributes: HashMap<&'static str, HashSet<&'static str>>,
}

impl Converter {
    pub fn new() -> Self {
        let mut element_attributes = HashMap::new();
        for e in ELEMENT_ATTRIBUTES {
            let attributes: Vec<&str> = e.1.split(',').collect();
            element_attributes.insert(e.0, HashSet::from_iter(attributes));
        }
        Converter {
            globals: HashSet::from(GLOBALS),
            event_handler: HashSet::from(EVENT_HANDLER),
            element_attributes,
        }
    }

    ///Convert `Target` to HTML String.
    pub fn convert(&self, target: Target) -> String {
        let mut rst = String::new();
        self.convert_target(&mut rst, target);
        rst
    }

    //"global attributes" or "event handlers" or "custom data attribute"
    fn is_attr(&self, str: &str) -> bool {
        self.globals.contains(str) || self.event_handler.contains(str) || str.starts_with(DATA)
    }

    //attributes on the element
    fn is_element_attr(&self, element: &str, s: &str) -> bool {
        if let Some(attributes) = self.element_attributes.get(element.to_lowercase().as_str()) {
            return attributes.contains(s);
        }
        false
    }

    fn convert_target(&self, rst: &mut String, target: Target) {
        let n = &target.name;
        for t in target.text {
            add_tag(rst, n, &t);
        }
        let mut attr_str = String::new();
        let mut child_str = String::new();
        for child in target.value {
            let c = &child.name;
            if (self.is_attr(c) && !is_element(n, c)) || self.is_element_attr(n, c) {
                add_attribute(&mut attr_str, c, &child.text);
            } else {
                self.convert_target(&mut child_str, child);
            }
        }
        if attr_str.len() > 0 || child_str.len() > 0 {
            add_tag_with_attribute(rst, n, &attr_str, &child_str);
        }
    }
}
