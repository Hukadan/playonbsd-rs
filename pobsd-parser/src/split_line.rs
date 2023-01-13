pub fn split_line(line: &str) -> (Option<&str>, Option<&str>) {
    let split_line: Vec<&str> = line.split('\t').collect();
    let left: Option<&str>;
    let right: Option<&str>;
    // split the line in a left and right hand sides
    match split_line.len() {
        1 => {
            if !split_line[0].is_empty() {
                left = Some(split_line[0]);
            } else {
                left = None;
            }
            right = None;
        }
        2 => {
            left = Some(split_line[0]);
            if !split_line[1].is_empty() {
                right = Some(split_line[1]);
            } else {
                right = None
            }
        }
        _ => {
            left = Some(split_line[0]);
            right = Some(split_line[1]);
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
        assert_eq!((Some("one"), Some("tab")), split_line(&test_str));
    }
}
