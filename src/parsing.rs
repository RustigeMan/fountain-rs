use crate::markup::ElmType;
use crate::reading::StringSavingBytesReader;
use crate::util::is_ascii_char;

use std::fs::File;
use std::io;
use std::io::Read;

pub fn parse_str(text: &str, offset: usize) -> Vec<IntElement> {
    let lines = CharParser::new(text.as_bytes().bytes(), offset).peekable();

    let markup = parse_lines(lines, text, offset).unwrap(); // No IO errors on in-memory str

    markup
}

pub fn parse_file(file: File) -> io::Result<(Vec<IntElement>, String)> {
    let metadata = file.metadata()?;
    let mut text = String::with_capacity(metadata.len() as usize);

    let markup = unsafe {
        // Because we're defining it inside this scope, the reader should
        // always go out of scope before the destination string. That way
        // it can never try to append to the string after it is dropped:
        let reader = StringSavingBytesReader::new(file.bytes(), &mut text);
        let lines = CharParser::new(reader, 0).peekable();

        parse_lines(lines, &(text), 0)?
    };

    Ok((markup, text))
}

pub fn parse_reader<R>(reader: R) -> io::Result<(Vec<IntElement>, String)>
where
    R: Read,
{
    let mut text = String::new();

    let markup = unsafe {
        // See fn parse_file() about unsafe usage of StringSavingBytesReader
        let reader = StringSavingBytesReader::new(reader.bytes(), &mut text);
        let lines = CharParser::new(reader, 0).peekable();

        parse_lines(lines, &(text), 0)?
    };

    Ok((markup, text))
}

fn parse_lines<'s, L>(
    mut lines: std::iter::Peekable<L>,
    text: impl AsRef<str>,
    offset: usize,
) -> io::Result<Vec<IntElement>>
where
    L: Iterator<Item = io::Result<LineStatus>>,
{
    let mut markup: Vec<IntElement> = Vec::new();
    let empty = LineStatus::new();
    let mut prev = LineStatus::new();
    while let Some(Ok(mut lstat)) = lines.next() {
        let next = if let Some(Ok(lstat)) = lines.peek() {
            lstat
        } else {
            &empty
        };

        if let (Some(start), Some(end)) = (lstat.start, lstat.end) {
            let range = start - offset..=end - offset;
            let l = text.as_ref().get(range).unwrap();

            let line_type = if l.starts_with("!") {
                ElmType::Action
            } else if l.starts_with("@") {
                ElmType::Character
            } else if l.starts_with(".") {
                ElmType::Heading
            } else if prev.all_whitespace {
                if next.all_whitespace {
                    if l.starts_with("INT") || l.starts_with("EXT") || l.starts_with("I/E") {
                        ElmType::Heading
                    } else if lstat.all_uppercase && l.ends_with("TO:") {
                        ElmType::Transition
                    } else {
                        ElmType::Action
                    }
                } else if lstat.all_uppercase {
                    ElmType::Character
                } else {
                    ElmType::Action
                }
            } else if let Some(ElmType::Character)
            | Some(ElmType::Dialogue)
            | Some(ElmType::Parenthetical) = prev.line_type
            {
                if l.starts_with('(') && l.ends_with(')') {
                    ElmType::Parenthetical
                } else {
                    ElmType::Dialogue
                }
            } else {
                ElmType::Action
            };
            lstat.line_type = Some(line_type);
            markup.push(IntElement::new(start, end, line_type));
        }

        if let Some(mut line_markup) = lstat.markup.take() {
            markup.append(&mut line_markup);
        }

        prev = lstat;
    }

    Ok(markup)
}

struct CharParser<B>
where
    B: Iterator<Item = io::Result<u8>>,
{
    bytes: B,
    offset: usize,
    cstat: CharStatus,
}

impl<B> CharParser<B>
where
    B: Iterator<Item = io::Result<u8>>,
{
    fn new(bytes: B, offset: usize) -> Self {
        Self {
            bytes,
            offset,
            cstat: CharStatus::new(),
        }
    }
}

impl<B> Iterator for CharParser<B>
where
    B: Iterator<Item = io::Result<u8>>,
{
    type Item = io::Result<LineStatus>;

    fn next(&mut self) -> Option<Self::Item> {
        let cstat = &mut self.cstat;
        let mut lstat = LineStatus::new();
        while let Some(result) = self.bytes.next() {
            let byte = match result {
                Ok(byte) => byte,
                Err(error) => return Some(Err(error)),
            };
            let i = self.offset;
            self.offset += 1;
            if let Some(start) = cstat.bnyd_start {
                if cstat.prev == '*' as u8 && byte == '/' as u8 {
                    lstat.push_markup(IntElement::bnyd(start, i));
                    cstat.bnyd_start = None;
                }
            } else if is_ascii_char(byte) {
                let ch = byte as char;

                match ch {
                    '\n' => {
                        self.cstat.newline_reset();

                        return Some(Ok(lstat));
                    }
                    'a'..='z' => lstat.all_uppercase = false,
                    _ => {}
                }

                if cstat.undl_before && (ch.is_ascii_whitespace() || ch.is_ascii_punctuation()) {
                    if let Some(start) = cstat.undl_start {
                        lstat.push_markup(IntElement::undl(start, i - 1));
                        cstat.undl_start = None;
                    }
                }
                if ch == '_' {
                    if cstat.undl_start == None && cstat.whspc_before {
                        cstat.undl_start = Some(i);
                    } else {
                        cstat.undl_before = true;
                    }
                } else {
                    cstat.undl_before = false;
                }

                if ch == '*' {
                    if cstat.prev == '/' as u8 {
                        cstat.bnyd_start = Some(i - 1);
                    }
                    if cstat.whspc_before {
                        if cstat.opening_stars_before == 0 {
                            cstat.opening_stars_before = 1;
                        }
                    } else {
                        if cstat.closing_stars_before == 0 {
                            cstat.closing_stars_before = 1;
                        }
                    }

                    if cstat.opening_stars_before > 0 && cstat.opening_stars_before < 3 {
                        cstat.opening_stars_before += 1;
                    }
                    if cstat.closing_stars_before > 0 && cstat.closing_stars_before < 3 {
                        cstat.closing_stars_before += 1;
                    }
                } else {
                    if cstat.closing_stars_before > 0
                        && (ch.is_ascii_whitespace() || ch.is_ascii_punctuation())
                    {
                        match cstat {
                            CharStatus {
                                closing_stars_before: 3,
                                boit_start: Some(start),
                                ..
                            } => {
                                lstat.push_markup(IntElement::boit(*start, i - 1));
                                cstat.boit_start = None;
                            }
                            CharStatus {
                                closing_stars_before: 2 | 3,
                                bold_start: Some(start),
                                ..
                            } => {
                                lstat.push_markup(IntElement::bold(*start, i - 1));
                                cstat.bold_start = None;
                            }
                            CharStatus {
                                closing_stars_before: 1 | 2 | 3,
                                ital_start: Some(start),
                                ..
                            } => {
                                lstat.push_markup(IntElement::ital(*start, i - 1));
                                cstat.ital_start = None;
                            }
                            _ => {}
                        };
                    }

                    cstat.opening_stars_before = 0;
                    cstat.closing_stars_before = 0;
                }

                if ch.is_ascii_whitespace() {
                    cstat.whspc_before = true;
                } else {
                    cstat.whspc_before = false;

                    if lstat.all_whitespace {
                        lstat.all_whitespace = false;
                        lstat.start = Some(i);
                    }
                    lstat.end = Some(i);
                }
            }

            cstat.prev = byte;
        }

        return if lstat.all_whitespace {
            None
        } else {
            Some(Ok(lstat))
        };
    }
}

#[derive(Clone, Debug)]
struct LineStatus {
    all_uppercase: bool,
    all_whitespace: bool,
    start: Option<usize>,
    end: Option<usize>,
    markup: Option<Vec<IntElement>>,
    line_type: Option<ElmType>,
}

struct CharStatus {
    prev: u8,
    bnyd_start: Option<usize>,
    ital_start: Option<usize>,
    bold_start: Option<usize>,
    boit_start: Option<usize>,
    undl_start: Option<usize>,
    undl_before: bool,
    opening_stars_before: usize,
    closing_stars_before: usize,
    whspc_before: bool,
}

impl CharStatus {
    pub fn new() -> Self {
        Self {
            prev: '\n' as u8,
            bnyd_start: None,
            ital_start: None,
            bold_start: None,
            boit_start: None,
            undl_start: None,
            undl_before: false,
            opening_stars_before: 0,
            closing_stars_before: 0,
            whspc_before: false,
        }
    }

    pub fn newline_reset(&mut self) {
        self.prev = '\n' as u8;
        self.ital_start = None;
        self.bold_start = None;
        self.boit_start = None;
        self.undl_start = None;
        self.opening_stars_before = 0;
        self.closing_stars_before = 0;
        self.undl_before = false;
        self.whspc_before = true;
    }
}

impl LineStatus {
    fn new() -> Self {
        LineStatus {
            all_uppercase: true,
            all_whitespace: true,
            start: None,
            end: None,
            markup: None,
            line_type: None,
        }
    }

    fn push_markup(&mut self, markup_span: IntElement) {
        if let None = self.markup {
            self.markup = Some(Vec::new());
        }

        if let Some(ref mut markup) = self.markup {
            markup.push(markup_span);
        } else {
            unreachable!();
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntElement {
    pub elm_type: ElmType,
    pub start: usize,
    pub end: usize,
}

impl IntElement {
    pub fn new(start: usize, end: usize, elm_type: ElmType) -> Self {
        Self {
            elm_type,
            start,
            end,
        }
    }

    pub fn bnyd(start: usize, end: usize) -> Self {
        Self::new(start, end, ElmType::Boneyard)
    }
    pub fn ital(start: usize, end: usize) -> Self {
        Self::new(start, end, ElmType::Italic)
    }
    pub fn bold(start: usize, end: usize) -> Self {
        Self::new(start, end, ElmType::Bold)
    }
    pub fn boit(start: usize, end: usize) -> Self {
        Self::new(start, end, ElmType::BoldItalic)
    }
    pub fn undl(start: usize, end: usize) -> Self {
        Self::new(start, end, ElmType::Underline)
    }
}
