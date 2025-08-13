pub fn is_armstrong_number(num: u32) -> bool {
    let num_len = num.to_string().len() as u32;

    let mut sum = 0;
    let mut rest = num;
    while rest > 0 {
        sum += (rest % 10).pow(num_len);
        rest /= 10;
    }

    num == sum
}
