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
