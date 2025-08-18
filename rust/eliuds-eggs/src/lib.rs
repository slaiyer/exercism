pub fn egg_count(display_value: u32) -> usize {
    (0..u32::BITS)
        .filter(|i| (display_value >> i) & 1 == 1)
        .count()
}
