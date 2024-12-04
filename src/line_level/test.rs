use super::get_line_level_text_edits;

#[test]
fn test_get_line_level_text_edits() {
    let text_edits =
        get_line_level_text_edits("aaa\nbbb".to_string(), "aac\nbbb\nccc\nddd\n".to_string())
            .unwrap();
    let text_edit = &text_edits[0];
    assert_eq!(text_edit.range.start.line, 0);
    assert_eq!(text_edit.range.start.character, 0);
    assert_eq!(text_edit.range.end.line, 1);
    assert_eq!(text_edit.range.end.character, 3);
}
