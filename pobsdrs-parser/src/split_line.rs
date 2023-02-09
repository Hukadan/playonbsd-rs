pub fn split_line(line: &str) -> (Option<&str>, Option<&str>) {
    // split the line in a left and right hand sides
    if line.is_empty() {
        return (None, None);
    }
    match line.split_once("\t") {
        None => (Some(line), None),
        Some((left, right)) => {
            if right.is_empty() {
                return (Some(left), None);
            }
            (Some(left), Some(right))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let test_str = "";
        assert_eq!((None, None), split_line(&test_str));
    }
    #[test]
    fn test_no_tab() {
        let test_str = "notab";
        assert_eq!((Some("notab"), None), split_line(&test_str));
    }
    #[test]
    fn test_no_tab_space() {
        let test_str = "no tab";
        assert_eq!((Some("no tab"), None), split_line(&test_str));
    }
    #[test]
    fn test_one_tab() {
        let test_str = "one\ttab";
        assert_eq!((Some("one"), Some("tab")), split_line(&test_str));
    }
    #[test]
    fn test_two_tab() {
        let test_str = "one\ttab\tanother";
        assert_eq!((Some("one"), Some("tab\tanother")), split_line(&test_str));
    }
}
