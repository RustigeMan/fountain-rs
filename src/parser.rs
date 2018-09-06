/*
TODO: Make sure some spans, like bones, don't contain other (/unallowed) spans.
*/

//use SpanType;

use std::collections::VecDeque;
use std::iter::Peekable;
use std::ops::Index;
use std::str::CharIndices;

use internal_span::IntSpan;

pub fn parse(chars: &str) -> Vec<IntSpan> {
    let mut int_spans = parse_chars(chars);
    let mut elements = parse_lines(chars, &int_spans);

    int_spans.append(&mut elements);
    int_spans.sort();
    int_spans
}

fn parse_chars(chars: &str) -> Vec<IntSpan> {
    let mut int_spans = Vec::new();
    int_spans.push(IntSpan::new_doc(0, chars.len()));
    let mut chars = PeekQue::from_iter(chars.char_indices());
    //let mut chars = chars.char_indices().peekable();

    let mut line_beg = 0;
    let mut bone_beg = None;
    let mut note_beg = None;
    //let mut blank_line_before = true;
    //let mut whitespace_before = true;
    //let mut emph_beg = None;
    let mut undl_beg = None;
    let mut emph_beg: [Option<usize>; 3] = [None, None, None];
    let _ = chars.fill_que(3);

    let mut last_char = '\n';
    let mut last_ind = 0;

    while let Some((i, ch)) = chars.next() {
        match ch {
            '\n' => {
                int_spans.push(IntSpan::new_line(line_beg, i - line_beg));

                //if line_beg == i {
                //    blank_line_before = true;
                //}

                undl_beg = None;
                line_beg = i + 1;
            }
            _ => {
                //ws_or_star_before = ch.is_whitespace();
            }
        }

        if chars.que_len() >= 1 {
            let q1 = chars[0];
            match (ch, q1.1) {
                ('/', '*') => if bone_beg == None {
                    bone_beg = Some(i);
                },
                ('*', '/') => if let Some(bindex) = bone_beg {
                    let length = q1.0 - bindex;
                    int_spans.push(IntSpan::new_bone(bindex, length));

                    bone_beg = None;
                    //blank_line_before = false;
                },
                ('[', '[') => if note_beg == None && bone_beg == None {
                    note_beg = Some(i)
                },
                (']', ']') => if let Some(nindex) = note_beg {
                    let length = q1.0 - nindex;
                    int_spans.push(IntSpan::new_note(nindex, length));

                    note_beg = None;
                    //blank_line_before = false;
                },
                (ch, '_') => if undl_beg == None && ch.is_whitespace() && bone_beg == None {
                    undl_beg = Some(q1.0);
                },
                ('_', ch) => if let Some(uindex) = undl_beg {
                    if ch.is_whitespace() && uindex != i {
                        int_spans.push(IntSpan::new_underline(uindex, i - uindex));
                        undl_beg = None;
                    }
                },
                _ => {}
            }
        }

        if ch.is_whitespace() {
            let stars = stars_in_a_row(&chars);
            if stars > 0 {
                if emph_beg[stars - 1] == None {
                    let first_star_ind = chars[0].0;
                    emph_beg[stars - 1] = Some(first_star_ind);
                }
            }
        }

        if ch == '*' {
            if let Some(stars) = ws_after_stars(&chars) {
                if let Some(eindex) = emph_beg[stars] {
                    let emph = stars + 1;
                    let length = i + stars - eindex;
                    int_spans.push(IntSpan::new_emphasis(emph as u8, eindex, length));

                    emph_beg[stars] = None;
                }
            }
        }

        if chars.que_len() == 0 && ch == '_' {
            if let Some(uindex) = undl_beg {
                int_spans.push(IntSpan::new_underline(uindex, i - uindex));
            }
        }

        last_char = ch;
        last_ind = i;
        //let whitespace_before = ch.is_whitespace();
    }

    if last_char != '\n' {
        int_spans.push(IntSpan::new_line(line_beg, last_ind - line_beg));
    }

    int_spans
}

fn stars_in_a_row(chars: &PeekQue) -> usize {
    let mut stars = 0;
    for i in 0..chars.que_len() {
        if chars[i].1 == '*' {
            stars += 1;
        } else {
            return stars;
        }
    }

    return stars;
}

fn ws_after_stars(chars: &PeekQue) -> Option<usize> {
    let mut stars = 0;
    for i in 0..chars.que_len() {
        let ch = chars[i].1;

        if ch == '*' {
            stars += 1;
        } else if ch.is_whitespace() {
            return Some(stars);
        } else {
            return None;
        }
    }

    return None;
}

fn parse_lines(chars: &str, spans: &Vec<IntSpan>) -> Vec<IntSpan> {
    let mut elements = Vec::new();

    //let mut prev_line_blank = true;
    let mut lines = spans.into_iter().filter(IntSpan::is_line).peekable();
    while let Some(line) = lines.next() {
        let line_str = line.str_from(chars);
        let trimmed = line_str.trim();

        let ind = line.index;
        let len = line.length;

        // If the line starts with ./!/~/@, or starts with > and
        // ends with <, this ovverides any other line parsing:
        if trimmed.len() == 0 {
        } else if trimmed.starts_with('.') {
            elements.push(IntSpan::new_heading(ind, len));
        } else if trimmed.starts_with('!') {
            elements.push(IntSpan::new_action(ind, len));
        } else if trimmed.starts_with('~') {
            elements.push(IntSpan::new_lyric(ind, len));
        } else if trimmed.starts_with('@') {
            parse_dialogue(line, &mut lines, chars, &mut elements);
        } else if trimmed.starts_with('>') {
            if trimmed.ends_with('<') {
                elements.push(IntSpan::new_centered(ind, len));
            } else {
                elements.push(IntSpan::new_transition(ind, len));
            }

        // Whole line parsing:
        } else if line_is_page_break(line_str) {
            elements.push(IntSpan::new_page_break(ind, len));
        } else if line_is_uppercase(line_str) {
            // Parsing dialogue is slightly more complicated:
            parse_dialogue(line, &mut lines, chars, &mut elements);
        } else if line_is_heading(line_str) {
            elements.push(IntSpan::new_heading(ind, len))
        } else {
            elements.push(IntSpan::new_action(ind, len));
        }
    }

    elements
}

/// Parses multiple lines of dialogue
fn parse_dialogue<'a, I>(
    line: &IntSpan,
    lines: &mut Peekable<I>,
    chars: &str,
    elements: &mut Vec<IntSpan>,
) where
    I: Iterator<Item = &'a IntSpan>,
{
    elements.push(IntSpan::new_character(line.index, line.length));

    while let Some(line) = lines.next() {
        let line_str = line.str_from(chars);

        if line_is_parenthetical(line_str) {
            elements.push(IntSpan::new_parenthetical(line.index, line.length));
        } else {
            let index = line.index;
            let mut length = line.length;
            while next_line_is_dialogue(lines, chars) {
                let line = lines.next().unwrap();
                length += line.length;
            }
            elements.push(IntSpan::new_dialogue(index, length));
        }

        if next_line_is_empty(lines, chars) {
            return;
        }
    }
}

fn line_is_uppercase(line: &str) -> bool {
    None == line.find(char::is_lowercase) && if let Some(_) = line.find(char::is_uppercase) {
        true
    } else {
        false
    }
}

fn line_is_parenthetical(line: &str) -> bool {
    line.starts_with('(') && line.ends_with(')')
}

fn line_is_heading(line: &str) -> bool {
    line.starts_with("INT.") || line.starts_with("EXT.")
}

fn line_is_page_break(line: &str) -> bool {
    line.len() >= 3 && None == line.trim_right().find(|c| c != '=')
}

fn next_line_is_empty<'a, I>(lines: &mut Peekable<I>, chars: &str) -> bool
where
    I: Iterator<Item = &'a IntSpan>,
{
    if let Some(line) = lines.peek() {
        let next_line = chars.get(line.index..line.length).unwrap();

        next_line.trim() == ""
    } else {
        true
    }
}

fn next_line_is_dialogue<'a, I>(lines: &mut Peekable<I>, chars: &str) -> bool
where
    I: Iterator<Item = &'a IntSpan>,
{
    if let Some(next_line) = lines.peek() {
        let line_str = next_line.str_from(chars);
        let trimmed = line_str.trim();

        !line_is_parenthetical(trimmed) && trimmed != ""
            || trimmed == "" && line_str.starts_with("  ")
    } else {
        false
    }
}

/// Similar to Peekable, but with a queue that can be filled to any size.
pub struct PeekQue<'a> {
    iter: CharIndices<'a>,
    queue: VecDeque<(usize, char)>,
}

impl<'a> PeekQue<'a> {
    fn from_iter(iter: CharIndices<'a>) -> Self {
        Self {
            iter: iter,
            queue: VecDeque::new(),
        }
    }

    fn que_len(&self) -> usize {
        self.queue.len()
    }

    fn shift_que(&mut self) -> Option<(usize, char)> {
        if let Some(char_ind) = self.iter.next() {
            self.queue.push_back(char_ind);
        }

        self.queue.pop_front()
    }

    fn fill_que(&mut self, size: usize) -> Result<(), ()> {
        while self.queue.len() < size {
            if let Some(char_ind) = self.iter.next() {
                self.queue.push_back(char_ind);
            } else {
                return Err(());
            }
        }

        return Ok(());
    }
}

impl<'a> Index<usize> for PeekQue<'a> {
    type Output = (usize, char);

    fn index(&self, i: usize) -> &Self::Output {
        &self.queue[i]
    }
}

impl<'a> Iterator for PeekQue<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.shift_que()
    }
}
