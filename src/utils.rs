use chrono::Local;

pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_date(d: chrono::DateTime<Local>) -> String {
    return d.format("%Y-%m-%d %H:%M:%S").to_string();
}