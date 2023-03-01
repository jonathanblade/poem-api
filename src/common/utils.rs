pub fn capitalize_string(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_error_message(msg: String) -> String {
    let msg = capitalize_string(msg);
    if !str::ends_with(&msg, ".") {
        return msg + ".";
    }
    return msg;
}
