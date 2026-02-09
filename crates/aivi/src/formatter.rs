pub fn format_text(content: &str) -> String {
    let mut output = String::new();
    let mut depth: isize = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            output.push('\n');
            continue;
        }

        let delta = brace_delta(trimmed);
        if delta < 0 {
            depth = (depth + delta).max(0);
        }

        let indent = "  ".repeat(depth as usize);
        output.push_str(&indent);
        output.push_str(trimmed);
        output.push('\n');

        if delta > 0 {
            depth += delta;
        }
    }

    output
}

fn brace_delta(line: &str) -> isize {
    let mut delta = 0;
    let mut chars = line.chars().peekable();
    let mut in_string = false;
    while let Some(ch) = chars.next() {
        if !in_string && ch == '/' && matches!(chars.peek(), Some('/')) {
            break;
        }
        if !in_string && ch == '-' && matches!(chars.peek(), Some('-')) {
            break;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if !in_string {
            if ch == '{' {
                delta += 1;
            } else if ch == '}' {
                delta -= 1;
            }
        }
    }
    delta
}
