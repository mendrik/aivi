use crate::lexer::lex;
use crate::syntax;

pub fn format_text(content: &str) -> String {
    let (tokens, _) = lex(content);
    let mut output = String::new();
    let mut depth: isize = 0;
    let mut last_last_kind = "";
    let mut last_last_text = "";
    let mut last_kind = "";
    let mut last_text = "";
    let mut last_line = 0;
    let mut newline_pending = false;

    for (i, token) in tokens.iter().enumerate() {
        let kind = token.kind.as_str();
        let text = token.text.as_str();
        let line = token.span.start.line;

        if kind == "whitespace" {
            continue;
        }

        // Detect newlines based on source lines
        if i > 0 && line > last_line {
            newline_pending = true;
        }

        // Before printing token: decrease depth for closing symbols
        if text == "}" || text == ")" || text == "]" {
            depth = (depth - 1).max(0);
        }

        if newline_pending {
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(&"  ".repeat(depth as usize));
            newline_pending = false;
        } else if should_add_space(
            last_last_kind,
            last_last_text,
            last_kind,
            last_text,
            kind,
            text,
        ) {
            output.push(' ');
        }

        if kind == "comment" {
            if !output.is_empty() && !output.ends_with('\n') && !output.ends_with(' ') {
                output.push(' ');
            }
            output.push_str(text);
            newline_pending = true; // Force newline after comment

            last_last_kind = last_kind;
            last_last_text = last_text;
            last_kind = kind;
            last_text = text;
            last_line = line; // Use current line effectively
            continue;
        }

        output.push_str(text);

        // After printing token: increase depth for opening symbols
        if text == "{" || text == "(" || text == "[" {
            depth += 1;
            let mut j = i + 1;
            while j < tokens.len() && tokens[j].kind == "whitespace" {
                j += 1;
            }
            if j < tokens.len() && tokens[j].span.start.line > line {
                newline_pending = true;
            }
        }

        last_last_kind = last_kind;
        last_last_text = last_text;
        last_kind = kind;
        last_text = text;
        last_line = line;
    }

    if !output.ends_with('\n') {
        output.push('\n');
    }

    output
}

fn should_add_space(
    last_last_kind: &str,
    last_last_text: &str,
    last_kind: &str,
    last_text: &str,
    current_kind: &str,
    current_text: &str,
) -> bool {
    if last_kind == ""
        || current_text == ","
        || current_text == ";"
        || current_text == "."
        || current_text == ":"
        || current_text == ")"
        || current_text == "]"
    {
        return false;
    }

    if last_text == "(" || last_text == "[" {
        return false;
    }

    if current_text == "}" {
        return last_text != "{";
    }

    if last_text == "{" {
        return current_text != "}";
    }

    // Date/Time fragments: no space around '-' or ':' if surrounded by numbers
    if current_text == "-" && last_kind == "number" {
        return false;
    }
    if last_text == "-" && last_last_kind == "number" {
        return false;
    }

    if current_text == ":" && last_kind == "number" {
        return false;
    }
    if last_text == ":" && last_last_kind == "number" && current_kind == "number" {
        return false;
    }

    // Unit suffixes: no space between number and ident/percent (except if ident is keyword)
    if last_kind == "number"
        && (current_text == "%" || (current_kind == "ident" && !is_keyword(current_text)))
    {
        return false;
    }

    let last_is_keyword = is_keyword(last_text);
    let current_is_keyword = is_keyword(current_text);

    if last_kind == "ident" && current_kind == "ident" {
        return true;
    }
    if last_is_keyword && current_kind == "ident" {
        return true;
    }
    if last_kind == "ident" && current_is_keyword {
        return true;
    }
    if last_is_keyword && current_is_keyword {
        return true;
    }
    if last_kind == "number" && current_kind == "ident" {
        return true;
    }

    // Operators
    if is_op(current_text) || current_text == "=" {
        return true;
    }
    if is_op(last_text) {
        if (last_text == "-" || last_text == "+") && !is_binary_precursor(last_last_text) {
            return false;
        }
        return true;
    }

    // After comma/colon
    if last_text == "," || last_text == ":" {
        return true;
    }

    false
}

fn is_binary_precursor(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    // Symbols that typically end an expression or literal
    if text == "}" || text == ")" || text == "]" {
        return true;
    }
    // Identifiers and numbers can be precursors to binary ops
    // We don't have kind here, but we can assume if it's not a symbol/keyword it's an ident/number
    !is_op(text) && text != "=" && text != "(" && text != "[" && text != "{" && text != "," && text != ":" && text != ";" && !is_keyword(text)
}

fn is_keyword(text: &str) -> bool {
    syntax::KEYWORDS_ALL.contains(&text)
}

fn is_op(text: &str) -> bool {
    matches!(
        text,
        "=" | "+"
            | "-"
            | "*"
            | "/"
            | "->"
            | "=>"
            | "<-"
            | "=="
            | "!="
            | "<"
            | ">"
            | "<="
            | ">="
            | "&&"
            | "||"
    )
}
