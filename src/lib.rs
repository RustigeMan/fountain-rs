mod markup;
mod parsing;
mod reading;
mod util;

#[cfg(test)]
mod tests;

pub use markup::*;

use parsing::{parse_file, parse_reader, parse_str, IntElement};

use std::fs::File;
use std::io;

pub struct Document {
    text: String,
    markup: Vec<IntElement>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            text: String::new(),
            markup: Vec::new(),
        }
    }

    pub fn from_file(file: File) -> io::Result<Self> {
        let (markup, text) = parse_file(file)?;

        Ok(Document { text, markup })
    }

    pub fn from_reader<R>(reader: R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let (markup, text) = parse_reader(reader)?;

        Ok(Document { text, markup })
    }

    pub fn elements(&self) -> Elements {
        Elements::new(self, 0, self.markup.len())
    }
}

impl<S> From<S> for Document
where
    S: Into<String>,
{
    fn from(text: S) -> Self {
        let text = text.into();
        let markup = parse_str(&text, 0);

        Document { text, markup }
    }
}

pub struct Elements<'d> {
    doc: &'d Document,
    index: usize,
    limit: usize,
}

impl<'d> Elements<'d> {
    pub fn new(doc: &'d Document, start: usize, end: usize) -> Self {
        Self {
            doc,
            index: start,
            limit: end,
        }
    }
    fn element_from_internal(&self, int_elm: &IntElement) -> Element<'d> {
        let text = unsafe { self.doc.text.get_unchecked(int_elm.start..=int_elm.end) };

        Element::new(int_elm.start, int_elm.elm_type, text)
    }
}

impl<'d> Iterator for Elements<'d> {
    type Item = Element<'d>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.limit {
            let i = self.index;
            self.index += 1;
            self.doc
                .markup
                .get(i)
                .map(|int_elm| self.element_from_internal(int_elm))
        } else {
            None
        }
    }
}
