#[cfg(test)]
mod tests {
    extern crate stringutility;
    use core::num;

    use stringutility::reverse_str;

    #[test]
    fn test_reverse_str_single_character() {
        let result = reverse_str("a");
        assert_eq!(result, "a");
    }

    #[test]
    fn test_reverse_str_two_characters() {
        let result = reverse_str("ab");
        assert_eq!(result, "ba");
    }

    #[test]
    fn test_reverse_str_three_characters() {
        let result = reverse_str("abc");
        assert_eq!(result, "cba");
    }

    #[test]
    fn test_reverse_str_with_capitalisation() {
        let result = reverse_str("AB");
        assert_eq!(result, "BA");
    }

    #[test]
    fn test_reverse_str_with_mixed_capitalisation() {
        let result = reverse_str("aBc");
        assert_eq!(result, "cBa");
    }

    #[test]
    fn test_reverse_str_with_special_characters() {
        let result = reverse_str("Hello!");
        assert_eq!(result, "!olleH");
    }

    #[test]
    fn test_reverse_str_with_special_numbers() {
        let result = reverse_str("hi1");
        assert_eq!(result, "1ih");
    }

    #[test]
    fn test_reverse_str_empty_str() {
        let result = reverse_str("");
        assert_eq!(result, "");
    }
}
