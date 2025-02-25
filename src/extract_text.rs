pub fn extract_numbers(text: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in text.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.is_empty() {
            
            if let Ok(num) = current_number.parse::<u32>() {
                numbers.push(num);
            }
            current_number.clear();
        }
    }

    // Check if there's a number at the end
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<u32>() {
            numbers.push(num);
        }
    }

    numbers
}
