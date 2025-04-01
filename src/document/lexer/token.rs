#[derive(Debug, PartialEq)]
struct DocTypeInner {
    name: Option<String>,              // TODO: Confirm type.
    public_identifier: Option<String>, // TODO: Confirm type.
    system_identifier: Option<String>, // TODO: Confirm type.
    force_quirks: bool,
}

#[derive(Debug, PartialEq)]
pub struct TagInner {
    pub tag_name: String,
    pub self_closing: bool,
    // What are the types of the attributes' "names" and "values".
    pub attrs: Vec<(String, String)>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    DocType(DocTypeInner),
    StartTag(TagInner),
    EndTag(TagInner),
    Comment(String),
    Character(char),
    EOF,
}

impl Default for DocTypeInner {
    fn default() -> Self {
        DocTypeInner {
            name: None,
            public_identifier: None,
            system_identifier: None,
            force_quirks: false,
        }
    }
}

impl Default for TagInner {
    fn default() -> Self {
        TagInner {
            tag_name: "".to_string(),
            self_closing: false,
            attrs: vec![],
        }
    }
}
