#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range,
    #[cfg_attr(feature = "serde", serde(rename = "newText"))]
    pub new_text: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}
