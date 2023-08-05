
    
pub fn capitalize_first_letter_of_each_word(input: &str) -> String {
    let mut result = String::new();
    
    let mut capitalize_next = true;
    
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c.to_ascii_lowercase());
            }
        } else {
            result.push(c);
            capitalize_next = c.is_whitespace();
        }
    }
    result
}
