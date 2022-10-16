pub fn split_line(line: &str) -> (&str, &str) {
    let split_line: Vec<&str> = line.split('\t').collect();
    let left: &str;
    let right: &str;
    // split the line in a left and right hand sides
    match split_line.len() {
        1 => {
            left = split_line[0];
            right = "";
        }
        2 => {
            left = split_line[0];
            right = split_line[1];
        }
        _ => {
            left = split_line[0];
            right = split_line[1];
            eprintln!(
                "WARNING: ignoring {}. Check tabs number in your database",
                split_line[2]
            );
        }
    };
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let test_str = "";
        assert_eq!(("", ""), split_line(&test_str));
    }
    #[test]
    fn test_no_tab() {
        let test_str = "notab";
        assert_eq!(("notab", ""), split_line(&test_str));
    }
    #[test]
    fn test_no_tab_space() {
        let test_str = "no tab";
        assert_eq!(("no tab", ""), split_line(&test_str));
    }
    #[test]
    fn test_one_tab() {
        let test_str = "one\ttab";
        assert_eq!(("one", "tab"), split_line(&test_str));
    }
    #[test]
    fn test_two_tab() {
        let test_str = "one\ttab\tanother";
        assert_eq!(("one", "tab"), split_line(&test_str));
    }
}
