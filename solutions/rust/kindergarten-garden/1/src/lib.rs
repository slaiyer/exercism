use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let map = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    if let Some(c) = student.as_bytes().first() {
        let i = 2 * (c.to_ascii_uppercase() - b'A');
        return diagram
            .lines()
            .flat_map(|line| line.chars().skip(i.into()).take(2))
            .map(|ref c| map[c])
            .collect();
    }

    Vec::new()
}
