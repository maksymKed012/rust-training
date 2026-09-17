use std::borrow::Cow;

fn compare_lines(line: &str, pattern: &str, ignore_case: bool, invert: bool) -> bool {
    let (line, pattern): (Cow<'_, str>, Cow<'_, str>) = if ignore_case {
        (
            Cow::Owned(line.to_lowercase()),
            Cow::Owned(pattern.to_lowercase()),
        )
    } else {
        (Cow::Borrowed(line), Cow::Borrowed(pattern))
    };

    let is_match = line.contains(pattern.as_ref());
    if invert { !is_match } else { is_match }
}

pub fn find_matches<'a>(
    pattern: &'a str,
    file_content: &'a str,
    ignore_case: bool,
    invert: bool,
) -> impl Iterator<Item = (usize, &'a str)> {
    file_content
        .lines()
        .enumerate()
        .filter(move |(_, line)| compare_lines(line, pattern, ignore_case, invert))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DUMMY_FILE: &str = "hi
    mom";
    #[test]
    fn find_matches_empty_pattern_expect_all_matches() {
        let mut result = find_matches("", DUMMY_FILE, true, false);
        assert_eq!(result.next().is_none(), false);
    }

    #[test]
    fn find_matches_not_matching_pattern_expect_no_matches() {
        let mut result = find_matches("kek", DUMMY_FILE, true, false);
        assert_eq!(result.next().is_none(), true);
    }

    #[test]
    fn find_matches_matching_pattern_expect_1_match() {
        let result = find_matches("hi", DUMMY_FILE, true, false);
        assert_eq!(result.count(), 1);
    }
    #[test]
    fn find_matches_other_matching_pattern_expect_1_match() {
        let result = find_matches("mom", DUMMY_FILE, true, false);
        assert_eq!(result.count(), 1);
    }
}
