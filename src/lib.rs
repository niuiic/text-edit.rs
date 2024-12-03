use similar::{ChangeTag, TextDiff};
use text_edit::TextEdit;

mod text_edit;

pub fn get_line_level_text_edits(
    old_text: String,
    new_text: String,
) -> Result<Vec<TextEdit>, String> {
    let diff = TextDiff::from_lines(&old_text, &new_text);

    let mut text_edits: Vec<TextEdit> = vec![];
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {}
            ChangeTag::Insert => {}
        }
    }
}
