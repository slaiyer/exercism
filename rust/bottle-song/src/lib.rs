pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let words = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    ((start_bottles - take_down + 1)..=start_bottles)
        .rev()
        .fold(String::new(), |acc, old| acc + &para(&words, old as usize))
}

fn para(words: &[&str], old: usize) -> String {
    let new = old - 1;
    let word_old = words[old];
    let word_new = words[new].to_lowercase();
    let bottle_suffix_old = if old > 1 { "s" } else { "" };
    let bottle_suffix_new = if new == 1 { "" } else { "s" };

    format!(
        "{word_old} green bottle{bottle_suffix_old} hanging on the wall,
{word_old} green bottle{bottle_suffix_old} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {word_new} green bottle{bottle_suffix_new} hanging on the wall.

",
    )
}
