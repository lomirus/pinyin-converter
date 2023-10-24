pub(crate) fn capitalize(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_ascii_uppercase().to_string() + c.as_str(),
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum WordCase {
    Capitalized,
    Lowercase,
    Uppercase,
}

impl From<&str> for WordCase {
    fn from(value: &str) -> Self {
        let value: Vec<char> = value.chars().collect();
        if value[0].is_lowercase() {
            WordCase::Lowercase
        } else if value[1].is_uppercase() {
            WordCase::Uppercase
        } else {
            WordCase::Capitalized
        }
    }
}
