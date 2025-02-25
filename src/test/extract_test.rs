#[cfg(test)]
mod tests {

    use crate::extract_text::extract_numbers;

    #[test]
    fn test_extract_numbers() {
        let text = "Numbers: 123, 456 and 789.";
        assert_eq!(extract_numbers(text), vec![123, 456, 789]);

        let text = "No numbers here!";
        assert_eq!(extract_numbers(text), vec![]);

        let text = "One number at the end 99";
        assert_eq!(extract_numbers(text), vec![99]);
    }
}
