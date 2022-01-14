
use std::str::Chars;


#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: usize,
    character: usize,
    abs: usize,
}

impl Position {
    pub fn new(line: usize, character: usize, abs: usize) -> Self {
        Position {
            line, character, abs
        }
    }
    pub fn init() -> Position {
        Position::new(0, 0, 0)
    }
}

impl std::ops::Add<char> for Position {
    type Output = Position;
    fn add(self, ch: char) -> Position {
        match ch {
            '\n' => Position{line: self.line+1, character: 0, abs: self.abs+1},
            _ => Position{line: self.line, character: self.character+1, abs: self.abs+1},
        }
    }
}
impl std::ops::Add<Chars<'_>> for Position {
    type Output = Position;
    fn add(self, chars: Chars<'_>) -> Position {
        let mut res = self;
        for ch in chars {
            res = res + ch;
        }
        res
    }
}

#[derive(Debug, PartialEq)]
pub struct Range<Position> {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub enum FileName {
    FileName(String),
    PseudoFile,
}

#[derive(Debug)]
pub struct Location<Position> {
    uri: FileName,
    range: Range<Position>,
}