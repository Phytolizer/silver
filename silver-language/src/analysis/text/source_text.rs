use std::ops::Index;

use super::{text_line::TextLine, text_span::TextSpan};

pub struct SourceText {
    text: String,
    lines: Vec<TextLine>,
}

impl SourceText {
    fn new(text: String) -> Self {
        Self {
            lines: Self::parse_lines(text.chars().collect()),
            text,
        }
    }

    fn parse_lines(text: Vec<char>) -> Vec<TextLine> {
        let mut result = vec![];

        let mut line_start = 0;
        let mut position = 0;
        while position < text.len() {
            let line_break_width = Self::get_line_break_width(&text, position);
            if line_break_width > 0 {
                result.push(TextLine::new(
                    line_start,
                    position,
                    position + line_break_width,
                ));
                position += line_break_width;
                line_start = position;
            } else {
                position += 1;
            }
        }
        if position > line_start {
            result.push(TextLine::new(line_start, position, position));
        }

        result
    }

    fn get_line_break_width(text: &[char], position: usize) -> usize {
        let current = text[position];
        let lookahead = text.get(position + 1).cloned().unwrap_or('\0');
        if current == '\r' && lookahead == '\n' {
            2
        } else if current == '\r' || current == '\n' {
            1
        } else {
            0
        }
    }

    pub fn get_line_index(&self, position: usize) -> usize {
        let mut lower = 0;
        let mut upper = self.lines.len() - 1;

        while lower <= upper {
            let index = lower + (upper - lower) / 2;
            let start = self.lines[index].start();

            if position == start {
                return index;
            }

            if start > position {
                upper = index - 1;
            } else {
                lower = index + 1;
            }
        }
        lower - 1
    }

    pub fn lines(&self) -> &[TextLine] {
        &self.lines
    }

    pub fn char_indices(&self) -> impl Iterator<Item = (usize, char)> + '_ {
        self.text.char_indices()
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl From<String> for SourceText {
    fn from(s: String) -> Self {
        Self::new(s.chars().collect())
    }
}

impl ToString for SourceText {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

impl Index<TextSpan> for SourceText {
    type Output = str;

    fn index(&self, index: TextSpan) -> &Self::Output {
        &self.text[index]
    }
}
