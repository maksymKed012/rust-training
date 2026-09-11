use std::borrow::Cow;

fn compare_lines(line: &str, pattern: &str, ignore_case: bool, invert: bool) -> bool {
    let mut pattern: Cow<'_, str> = Cow::Borrowed(pattern);
    if ignore_case {
        pattern = Cow::Owned(pattern.to_lowercase());
    }
    if invert {
        !line.contains(pattern.as_ref())
    } else {
        line.contains(pattern.as_ref())
    }
}

fn build_output_string(line: &str, line_number: i32, show_line_number: bool) -> String {
    let mut line_str_ = String::new();
    if show_line_number {
        line_str_ = format!("line: {}", line_number);
    }
    format!("{} {}", line_str_, line)
}

pub fn find_matches(
    pattern: &str,
    file_content: &str,
    ignore_case: bool,
    show_line_number: bool,
    invert: bool,
) {
    let mut file_content: Cow<'_, str> = Cow::Borrowed(file_content);
    if ignore_case {
        file_content = Cow::Owned(file_content.to_lowercase());
    }

    for (i, line) in file_content.lines().enumerate() {
        if compare_lines(line, pattern, ignore_case, invert) {
            let out_str = build_output_string(line, (i + 1) as i32, show_line_number);
            println!("match found: {out_str}");
        }
    }
}
