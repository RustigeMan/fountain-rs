/// A library for parsing Fountain markdown.

#[cfg(test)]
mod tests;

mod internal_span;
mod parser;

use std::string::ToString;

use internal_span::IntSpan;

#[derive(Debug, Eq, PartialEq)]
pub struct Doc {
    text_content: String,
    spans: Vec<IntSpan>,
}

/// Represents a Fountain document
impl Doc {
    pub fn new() -> Self {
        Doc {
            /// Unaltered text content
            text_content: String::new(),
            /// Metadata over slices of text
            spans: parser::parse(""),
        }
    }

    pub fn from_string(string: String) -> Self {
        let spans = parser::parse(&string);
        Doc {
            text_content: string,
            spans: spans,
        }
    }

    fn int_spans(&self) -> impl Iterator<Item = &IntSpan> {
        self.spans.iter()
    }

    pub fn spans(&self) -> impl Iterator<Item = Span> {
        self.spans.iter().map(move |int_sp| Span {
            play: self,
            int_span: int_sp,
        })
    }

    /// Get all spans that represent elements
    /// (action, header, character, dialogue, etc...)
    pub fn elements(&self) -> impl Iterator<Item = Element> {
        self.to_elements(self.int_spans().filter(IntSpan::is_element))
    }

    fn to_elements<'a>(
        &'a self,
        int_spans: impl Iterator<Item = &'a IntSpan>,
    ) -> impl Iterator<Item = Element> {
        int_spans.map(move |span| Element {
            play: self,
            int_span: span,
        })
    }

    /*
    fn int_span_str(&self, span: &IntSpan) -> &str {
        span.str_from(&self.text_content)
    }
    */

    pub fn range(&self, index: usize, length: usize) -> impl Iterator<Item = &IntSpan> {
        let beg = index;
        let end = index + length;

        self.spans
            .iter()
            .filter(move |span| span.index >= beg && span.index + span.length <= end)
    }
}

impl ToString for Doc {
    fn to_string(&self) -> String {
        self.text_content.clone()
    }
}

impl SpanLike for Doc {
    fn doc(&self) -> &Doc {
        &self
    }

    fn ind(&self) -> usize {
        0
    }

    fn len(&self) -> usize {
        self.text_content.len()
    }

    fn span(&self) -> Span {
        Span {
            play: self,
            int_span: &self.spans[0],
        }
    }

    fn span_type(&self) -> SpanType {
        SpanType::Doc
    }
}

pub trait SpanLike: Sized {
    fn doc(&self) -> &Doc;
    //fn doc_mut(&mut self) -> &mut Doc;

    fn ind(&self) -> usize;
    fn len(&self) -> usize;

    fn as_str(&self) -> &str {
        self.doc()
            .text_content
            .get(self.ind()..self.ind() + self.len())
            .unwrap()
    }

    fn span_type(&self) -> SpanType;

    fn positions(&self) -> Positions<Self> {
        Positions {
            span: self,
            index: 0,
        }
    }

    fn span(&self) -> Span;

    unsafe fn char_at(&self, index: usize) -> char {
        self.doc().text_content[self.ind() + index..]
            .chars()
            .next()
            .unwrap()
    }

    /*
    fn style<'a, I>(&self) -> I
    where
        I: Iterator<Item = Span<'a>>,
    {
        self.doc().range(self.ind(), self.len()).map(|s| AnySpan {
            play: self.doc(),
            _span: s,
        })
    }
    */
}

#[derive(Debug, Eq, PartialEq)]
pub struct Position<'a, S>
where
    S: SpanLike + 'a,
{
    span: &'a S,
    index: usize,
}

impl<'a, S> Position<'a, S>
where
    S: SpanLike + 'a,
{
    pub fn char(&self) -> char {
        unsafe { self.char_at(0) }
    }
}

impl<'a, S> SpanLike for Position<'a, S>
where
    S: SpanLike + 'a,
{
    fn ind(&self) -> usize {
        self.index
    }

    fn len(&self) -> usize {
        self.span.len()
    }

    fn span(&self) -> Span {
        self.span.span()
    }

    fn span_type(&self) -> SpanType {
        self.span.span_type()
    }

    fn doc(&self) -> &Doc {
        self.span.doc()
    }
}

pub struct Positions<'a, S>
where
    S: SpanLike + 'a,
{
    span: &'a S,
    index: usize,
}

impl<'a, S> Iterator for Positions<'a, S>
where
    S: SpanLike + 'a,
{
    type Item = Position<'a, S>;

    fn next(&mut self) -> Option<Position<'a, S>> {
        if self.index < self.span.len() {
            let pos = Position {
                span: self.span,
                index: self.index,
            };

            let charlen = unsafe { self.span.char_at(self.index).len_utf8() };
            self.index += charlen;

            Some(pos)
        } else {
            None
        }
    }
}

/*
impl ToString for Span {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
*/

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum SpanType {
    Doc,
    Line,
    Element(ElmType),
    Note,
    Boneyard,
    Emphasis(u8),
    Underline,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Span<'a> {
    play: &'a Doc,
    int_span: &'a IntSpan,
}

impl<'a> SpanLike for Span<'a> {
    fn doc(&self) -> &Doc {
        &self.play
    }

    fn ind(&self) -> usize {
        self.int_span.index
    }
    fn len(&self) -> usize {
        self.int_span.length
    }

    fn span(&self) -> Span {
        self.clone()
    }

    fn span_type(&self) -> SpanType {
        self.int_span.stype
    }
}

/// Represents a span over an element
#[derive(Debug, Eq, PartialEq)]
pub struct Element<'a> {
    play: &'a Doc,
    int_span: &'a IntSpan,
}

impl<'a> Element<'a> {
    pub fn elm_type(&self) -> ElmType {
        self.int_span.elm_type()
    }
}

impl<'a> SpanLike for Element<'a> {
    fn doc(&self) -> &Doc {
        &self.play
    }

    fn ind(&self) -> usize {
        self.int_span.index
    }
    fn len(&self) -> usize {
        self.int_span.length
    }

    fn span(&self) -> Span {
        Span {
            play: self.doc(),
            int_span: self.int_span,
        }
    }

    fn span_type(&self) -> SpanType {
        self.int_span.stype
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum ElmType {
    Action,
    Heading,
    Character,
    Parenthetical,
    Dialogue,
    Transition,
    PageBreak,
    Lyric,
    Centered,
}
