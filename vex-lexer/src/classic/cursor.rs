/// The Cursor provides structural navigation and lookahead operations for the lexical analyzer.
/// It encapsulates the character source buffer, current byte offsets, and line/column state
/// tracking to facilitate accurate metadata association for compiler diagnostics and token mapping.
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Cursor {
    /// Zero-indexed absolute offset representing the cursor's location within the file buffer.
    pub pos: usize,
    /// Absolute line coordinate (1-indexed base) tracking newline divisions.
    pub line: usize,
    /// Absolute column coordinate (1-indexed base) tracking lateral progression relative to the current line.
    pub col: usize,
    /// Memory-resident character buffer reflecting the exact sequence of the associated file.
    pub source: Vec<char>,
}

impl Cursor {
    /// Instantiates a new navigational context against the mapped source sequence.
    pub fn new(source: Vec<char>) -> Self {
        Self {
            pos: 0,
            line: 1,
            col: 1,
            source,
        }
    }

    /// Evaluates whether the cursor bounds have exceeded or met the constraints of the buffer allocation.
    pub fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// Retrieves the nominal value at the current positional index without propagating the state.
    /// Yields '\0' to safely indicate operational exhaustion constraints.
    pub fn peek(&self) -> char {
        self.peek_at(0)
    }

    /// Retrieves the nominal value natively localized at a +1 progression offset without propagation.
    pub fn peek_next(&self) -> char {
        self.peek_at(1)
    }

    /// Calculates sequence projections based on a parameterized absolute distance offset constraint.
    pub fn peek_at(&self, offset: usize) -> char {
        self.source.get(self.pos + offset).copied().unwrap_or('\0')
    }

    /// Yields the value localized at the current absolute offset constraint and mutates
    /// the relative coordinate metrics for downstream processing phases.
    pub fn advance(&mut self) -> char {
        let ch = self.peek();
        self.pos += 1;

        let is_nl = (ch == '\n') as usize;
        self.line += is_nl;
        self.col = (self.col * (1 - is_nl)) + 1;

        ch
    }

    /// Validates whether the sequentially targeted data correlates to the expected structural constraint.
    /// Conditionally executes an advance logic transition strictly upon successful resolution operations.
    pub fn match_char(&mut self, expected: char) -> bool {
        (self.peek() == expected && !self.is_at_end()).then(|| self.advance()).is_some()
    }
}
