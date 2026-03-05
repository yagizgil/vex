
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Span {
    pub file_id: usize,
    pub start: usize, // start index
    pub end: usize,   //end index
    pub line: usize,  // line number
    pub col: usize,   // colon number
}

impl Span {
    pub fn new(file_id: usize, start: usize, end: usize, line: usize, col: usize) -> Self {
        Self { file_id, start, end, line, col }
    }

    pub fn dummy() -> Self {
        Self { file_id: 0, start: 0, end: 0, line: 0, col: 0 }
    }
}