pub fn first_word(s: &str) -> &str {
    // Convert the string into bytes so we can check for spaces easily
    let bytes = s.as_bytes();

    // Iterate through each byte with its index
    for (i, &item) in bytes.iter().enumerate() {
        // If we find a space, return a slice from start to that position
        if item == b' ' {
            return &s[..i];
        }
    }

    // If no space is found, return the whole string
    &s[..]
}