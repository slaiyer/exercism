pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|w| String::from_utf8(w.to_owned()).unwrap())
        .collect::<Vec<_>>()
}
