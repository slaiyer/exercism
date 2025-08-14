pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => {
            list.windows(2)
                .map(|pair| sentence(pair[0], pair[1]))
                .collect::<String>()
                + &format!("And all for the want of a {}.", list[0])
        }
    }
}

fn sentence(first: &str, second: &str) -> String {
    format!("For want of a {first} the {second} was lost.\n")
}
