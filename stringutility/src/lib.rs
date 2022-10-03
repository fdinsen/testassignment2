const LOWERCASE_LOWER_BOUND: u8 = 97;
const LOWERCASE_UPPER_BOUND: u8 = 122;
const UPPERCASE_LOWER_BOUND: u8 = 65;
const UPPERCASE_UPPER_BOUND: u8 = 90;
const CASE_DIFFERENCE: u8 = 32;

pub fn reverse_str(input: &str) -> String {
    let mut output = String::new();
    for char in input.chars() {
        output = push_front(output, &char.to_string());
    }
    output
}

pub fn capitalize_str(input: &str) -> String {
    let mut output = String::new();
    let bytes = input.as_bytes().to_vec();
    for byte in bytes {
        let chararacter = 
            if byte >= LOWERCASE_LOWER_BOUND && byte <= LOWERCASE_UPPER_BOUND { // if is lower case letter
                (byte - CASE_DIFFERENCE) as char // subtract CASE_DIFFERENCE to make it upper case
            }else {
                byte as char
            };
        output.push(chararacter);
    }
    output
}

pub fn lowercase_str(input: &str) -> String {
    let mut output = String::new();
    let bytes = input.as_bytes().to_vec();
    for byte in bytes {
        let character = 
            if byte >= UPPERCASE_LOWER_BOUND && byte <= UPPERCASE_UPPER_BOUND { // if is upper case letter
                (byte + CASE_DIFFERENCE) as char // add CASE_DIFFERENCE to make it lower case
            }else {
                byte as char
            };
        output.push(character);
    }
    output
}

fn push_front(mut s: String, prefix: &str) -> String {
    s.insert_str(0, prefix);
    s
}