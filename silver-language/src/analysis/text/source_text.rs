use super::{text_line::TextLine, text_span::TextSpan};

pub struct SourceText {
    text: Vec<char>,
    lines: Vec<TextLine>,
}

impl SourceText {
    fn new(text: Vec<char>) -> Self {
        Self {
            lines: Self::parse_lines(&text),
            text,
        }
    }

    fn parse_lines(text: &[char]) -> Vec<TextLine> {
        let mut result = vec![];

        let mut line_start = 0;
        let mut position = 0;
        while position < text.len() {
            let line_break_width = Self::get_line_break_width(text, position);
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

        result
    }

    fn get_line_break_width(text: &[char], position: usize) -> usize {
        let c = text[position];
        let l = if position + 1 >= text.len() {
            '\0'
        } else {
            text[position + 1]
        };
        if c == '\r' && l == '\n' {
            2
        } else if c == '\r' || c == '\n' {
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

    pub fn to_string(&self, span: TextSpan) -> String {
        self.text[span].iter().collect()
    }
}

impl From<String> for SourceText {
    fn from(s: String) -> Self {
        Self::new(s.chars().collect())
    }
}

impl ToString for SourceText {
    fn to_string(&self) -> String {
        self.text.iter().collect()
    }
}
