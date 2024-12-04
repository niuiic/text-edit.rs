pub fn get_line_ending() -> &'static str {
    if cfg!(target_os = "windows") {
        "\r\n"
    } else if cfg!(target_os = "macos") && !cfg!(target_family = "unix") {
        "\r"
    } else {
        "\n"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_get_line_ending() {
        if cfg!(target_os = "linux") {
            assert_eq!(get_line_ending(), "\n");
        }
    }
}
