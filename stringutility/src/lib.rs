pub fn reverse_str(input: &str) -> String {
    let mut output = String::new();
    for char in input.chars() {
        output = push_front(output, &char.to_string());
    }
    output
}

fn push_front(mut s: String, prefix: &str) -> String {
    s.insert_str(0, prefix);
    s
}