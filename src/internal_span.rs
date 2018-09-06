use std::cmp;
use ElmType;
use SpanType;

/// Internal data structure shared between Doc struct and parser module
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IntSpan {
    pub index: usize,
    pub length: usize,
    pub stype: SpanType,
}

impl IntSpan {
    fn new(index: usize, length: usize, stype: SpanType) -> Self {
        Self {
            index: index,
            length: length,
            stype: stype,
        }
    }

    pub fn new_doc(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Doc)
    }
    pub fn new_line(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Line)
    }
    pub fn new_bone(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Boneyard)
    }
    pub fn new_note(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Note)
    }
    pub fn new_underline(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Underline)
    }
    pub fn new_emphasis(emph: u8, index: usize, length: usize) -> Self {
        if emph > 3 {
            panic!("Tried to create emphasis span with emph > 3")
        }
        Self::new(index, length, SpanType::Emphasis(emph))
    }
    pub fn new_heading(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Heading))
    }
    pub fn new_action(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Action))
    }
    pub fn new_lyric(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Lyric))
    }
    pub fn new_character(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Character))
    }
    pub fn new_parenthetical(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Parenthetical))
    }
    pub fn new_dialogue(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Dialogue))
    }
    pub fn new_transition(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Transition))
    }
    pub fn new_centered(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::Centered))
    }
    pub fn new_page_break(index: usize, length: usize) -> Self {
        Self::new(index, length, SpanType::Element(ElmType::PageBreak))
    }

    pub fn is_element(span: &&Self) -> bool {
        if let SpanType::Element(_) = span.stype {
            true
        } else {
            false
        }
    }

    pub fn is_line(span: &&Self) -> bool {
        span.stype == SpanType::Line
    }

    /// Will panic if called on _Span that's not an element.
    pub fn elm_type(&self) -> ElmType {
        if let SpanType::Element(etype) = self.stype {
            etype
        } else {
            panic!("Called .elm_type() on non-element: {:?}", self)
        }
    }

    pub fn str_from<'a>(&self, chars: &'a str) -> &'a str {
        chars.get(self.index..self.index + self.length).unwrap()
    }
}

impl Ord for IntSpan {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use std::cmp::Ordering::*;

        match self.index.cmp(&other.index) {
            Less => Less,
            Greater => Greater,
            Equal => match self.length.cmp(&other.length).reverse() {
                Less => Less,
                Greater => Greater,
                Equal => self.stype.cmp(&other.stype),
            },
        }
    }
}

impl PartialOrd for IntSpan {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
