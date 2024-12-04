use crate::{
    line_ending::get_line_ending,
    text_edit::{Position, Range, TextEdit},
};

#[cfg(test)]
mod test;

#[allow(dead_code)]
pub fn get_line_level_text_edits(
    old_text: String,
    new_text: String,
) -> Result<Vec<TextEdit>, String> {
    let old_lines: Vec<String> = old_text.lines().map(|x| x.to_string()).collect();
    let new_lines: Vec<String> = new_text.lines().map(|x| x.to_string()).collect();

    let diff_start_line_index: usize;
    match get_diff_start_line_index(&old_lines, &new_lines) {
        None => return Ok(vec![]),
        Some(x) => diff_start_line_index = x,
    }

    Ok(vec![get_text_edit(
        &old_lines,
        &new_lines,
        diff_start_line_index,
    )])
}

fn get_diff_start_line_index(old_lines: &[String], new_lines: &[String]) -> Option<usize> {
    if old_lines.len() == 0 && new_lines.len() == 0 {
        return None;
    }

    let line_count = old_lines.len().min(new_lines.len());
    if line_count == 0 {
        return Some(0);
    }

    let mut start_line_index: Option<usize> = None;
    for i in 0..line_count {
        if old_lines[i] != new_lines[i] {
            start_line_index = Some(i);
            break;
        }
    }

    if start_line_index.is_none() && (old_lines.len() > line_count || new_lines.len() > line_count)
    {
        start_line_index = Some(line_count);
    }

    start_line_index
}

fn get_text_edit(
    old_lines: &[String],
    new_lines: &[String],
    diff_start_line_index: usize,
) -> TextEdit {
    if old_lines.len() == diff_start_line_index + 1 && new_lines.len() == diff_start_line_index + 1
    {
        return TextEdit {
            range: Range {
                start: Position {
                    line: diff_start_line_index,
                    character: 0,
                },
                end: Position {
                    line: diff_start_line_index,
                    character: old_lines[diff_start_line_index].chars().count(),
                },
            },
            new_text: new_lines[diff_start_line_index].clone(),
        };
    }

    if old_lines.len() == diff_start_line_index + 1 && new_lines.len() > diff_start_line_index + 1 {
        return TextEdit {
            range: Range {
                start: Position {
                    line: diff_start_line_index,
                    character: 0,
                },
                end: Position {
                    line: diff_start_line_index,
                    character: old_lines[diff_start_line_index].chars().count(),
                },
            },
            new_text: new_lines[diff_start_line_index..].join(get_line_ending()),
        };
    }

    if old_lines.len() > diff_start_line_index + 1 && new_lines.len() == diff_start_line_index + 1 {
        return TextEdit {
            range: Range {
                start: Position {
                    line: diff_start_line_index,
                    character: 0,
                },
                end: Position {
                    line: old_lines.len() - 1,
                    character: old_lines[old_lines.len() - 1].chars().count(),
                },
            },
            new_text: new_lines[diff_start_line_index..].join(get_line_ending()),
        };
    }

    let rev_line_count = (old_lines.len() - diff_start_line_index - 1)
        .min(new_lines.len() - diff_start_line_index - 1);
    let mut rev_diff_end_line_num = rev_line_count + 1;
    for i in 1..=rev_line_count {
        if old_lines[old_lines.len() - i] != new_lines[new_lines.len() - i] {
            rev_diff_end_line_num = i;
            break;
        }
    }

    return TextEdit {
        range: Range {
            start: Position {
                line: diff_start_line_index,
                character: 0,
            },
            end: Position {
                line: old_lines.len() - rev_diff_end_line_num,
                character: old_lines[old_lines.len() - rev_diff_end_line_num]
                    .chars()
                    .count(),
            },
        },
        new_text: new_lines[diff_start_line_index..=new_lines.len() - rev_diff_end_line_num]
            .join(get_line_ending()),
    };
}
