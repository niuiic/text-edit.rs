#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}
