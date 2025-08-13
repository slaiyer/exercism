use std::collections::HashMap;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() || garden.iter().any(|r| r.len() != garden[0].len()) {
        return Vec::new();
    }

    let (r_max, c_max) = (garden.len() - 1, garden[0].len() - 1);
    let mut map = HashMap::new();

    for (r_i, r_str) in garden.iter().enumerate() {
        for (c_i, c) in r_str.as_bytes().iter().enumerate() {
            if c == &b'*' {
                update(
                    &mut map,
                    r_max as isize,
                    c_max as isize,
                    r_i as isize,
                    c_i as isize,
                )
            }
        }
    }

    bloom_repr(garden, map)
}

fn update(
    map: &mut HashMap<(usize, usize), i8>,
    r_max: isize,
    c_max: isize,
    r_i: isize,
    c_i: isize,
) {
    for i in -1..=1 {
        for j in -1..=1 {
            match (i, j) {
                (0, 0) => continue,
                _ => match (r_i + i, c_i + j) {
                    (m, n) if m < 0 || n < 0 || m > r_max || n > c_max => continue,
                    (m, n) => {
                        map.entry((m as usize, n as usize))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                },
            }
        }
    }
}

fn bloom_repr(garden: &[&str], map: HashMap<(usize, usize), i8>) -> Vec<String> {
    let mut repr: Vec<String> = Vec::with_capacity(garden.len());
    repr.iter_mut()
        .for_each(|r| *r = String::with_capacity(garden[0].len()));

    for (r_i, r_str) in garden.iter().enumerate() {
        for (c_i, c) in r_str.as_bytes().iter().enumerate() {
            match c {
                b'*' => repr[r_i] += "*",
                _ => repr[r_i] += &map[&(r_i, c_i)].to_string(),
            }
        }
    }

    repr
}
