#[cfg(test)]
mod tests {
    extern crate stringutility;

    use stringutility::{reverse_str, capitalize_str, lowercase_str};

    // Reverse String Tests
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

    // Lowercase String Tests
    #[test]
    fn test_lowercase_str_single_character() {
        let result = lowercase_str("A");
        assert_eq!(result, "a");
    }

    #[test]
    fn test_lowercase_str_two_characters() {
        let result = lowercase_str("AB");
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_lowercase_str_two_characters_varying_capitalisation() {
        let result = lowercase_str("AbC");
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_lowercase_str_special_character() {
        let result = lowercase_str("A!!!");
        assert_eq!(result, "a!!!");
    }

    #[test]
    fn test_lowercase_str_nubmer() {
        let result = lowercase_str("A123");
        assert_eq!(result, "a123");
    }

    #[test]
    fn test_lowercase_str_empty_string() {
        let result = lowercase_str("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_lowercase_str_with_space() {
        let result = lowercase_str("Hello, World!");
        assert_eq!(result, "hello, world!");
    }

    // Capitalize String Tests
    #[test]
    fn test_capitalize_str_single_character() {
        let result = capitalize_str("a");
        assert_eq!(result, "A");
    }
    #[test]
    fn test_capitalize_str_two_characters() {
        let result = capitalize_str("ac");
        assert_eq!(result, "AC");
    }
    #[test]
    fn test_capitalize_str_two_characters_varying_capitalisation() {
        let result = capitalize_str("aC");
        assert_eq!(result, "AC");
    }
    #[test]
    fn test_capitalize_str_special_characters() {
        let result = capitalize_str("!aC!");
        assert_eq!(result, "!AC!");
    }
    #[test]
    fn test_capitalize_str_number() {
        let result = capitalize_str("aC123");
        assert_eq!(result, "AC123");
    }
    #[test]
    fn test_capitalize_str_empty_string() {
        let result = capitalize_str("");
        assert_eq!(result, "");
    }
    #[test]
    fn test_capitalize_str_with_space() {
        let result = capitalize_str("Hello, world!");
        assert_eq!(result, "HELLO, WORLD!");
    }
}
