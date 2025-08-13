const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_neighbors(garden: &[&str], x: usize, y: usize) -> char {
    let (height, width) = (garden.len(), garden[0].len());
    match OFFSETS
        .iter()
        .map(|&(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
        .filter(|&(x, y)| x < width && y < height && garden[y].as_bytes()[x] == b'*')
        .count()
    {
        0 => ' ',
        n => (n as u8 + b'0') as char,
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.bytes()
                .enumerate()
                .map(|(x, cell)| match cell {
                    b'*' => '*',
                    _ => count_neighbors(garden, x, y),
                })
                .collect()
        })
        .collect()
}
