fn first_space_index(s: &str) -> Option<usize> {
    s.chars().position(|c| c == ' ')
}