#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElmType {
    Action,
    Character,
    Parenthetical,
    Dialogue,
    Heading,
    Transition,
    Bold,
    Italic,
    BoldItalic,
    Underline,
    Boneyard,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element<'s> {
    elm_type: ElmType,
    offset: usize,
    text: &'s str,
}

impl<'s> Element<'s> {
    pub fn new(offset: usize, elm_type: ElmType, text: &'s str) -> Self {
        Self {
            offset,
            text,
            elm_type,
        }
    }

    pub fn elm_type(&self) -> ElmType {
        self.elm_type
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
    pub fn text(&self) -> &'s str {
        self.text
    }
    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl<'s> std::fmt::Display for Element<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.text())
    }
}

#[derive(Clone, Debug)]
pub struct Slice<'s> {
    text: &'s str,
    offset: usize,
    markup: Vec<Element<'s>>,
}

impl<'s> Slice<'s> {
    pub fn new(offset: usize, text: &'s str, markup: Vec<Element<'s>>) -> Self {
        Self {
            offset,
            text,
            markup,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
    pub fn text(&self) -> &'s str {
        self.text
    }
    pub fn len(&self) -> usize {
        self.text.len()
    }
    pub fn markup(&self) -> &Vec<Element> {
        &self.markup
    }
}
