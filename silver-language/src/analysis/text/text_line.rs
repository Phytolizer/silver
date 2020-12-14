use super::text_span::TextSpan;

pub struct TextLine {
    start: usize,
    end: usize,
    end_including_line_break: usize,
}

impl TextLine {
    pub fn new(start: usize, end: usize, end_including_line_break: usize) -> Self {
        Self {
            start,
            end,
            end_including_line_break,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn end_including_line_break(&self) -> usize {
        self.end_including_line_break
    }

    pub fn span(&self) -> TextSpan {
        self.start..self.end
    }

    pub fn span_including_line_break(&self) -> TextSpan {
        self.start..self.end_including_line_break
    }
}
