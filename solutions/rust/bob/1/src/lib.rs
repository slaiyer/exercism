pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let mut alphabetic_chars = message.chars().filter(|c| c.is_alphabetic()).peekable();
    let is_yell = alphabetic_chars.peek().is_some() && alphabetic_chars.all(|c| c.is_uppercase());
    let is_question = message.ends_with('?');
    let is_silence = message.chars().all(|c| c.is_whitespace());

    match (is_yell, is_question, is_silence) {
        (_, _, true) => "Fine. Be that way!",
        (false, true, _) => "Sure.",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
