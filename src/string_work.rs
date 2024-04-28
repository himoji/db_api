pub fn extract_number(text: &str) -> Option<i64> {
    let mut number = String::new();
    let mut found_number = false;

    for c in text.chars() {
        if c.is_ascii_digit() {
            found_number = true;
            number.push(c);
        } else if found_number {
            break; // Stop if we've found the number and encountered a non-digit character
        }
    }

    if found_number {
        number.parse().ok()
    } else {
        None
    }
}
