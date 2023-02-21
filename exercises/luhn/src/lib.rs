/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut len = 0;
    let digit_sum = code
        .chars()
        .rev()
        .filter(|char| !char.is_whitespace())
        .enumerate()
        .try_fold(0, |sum, (i, num)| {
            len += 1;
            num.to_digit(10)
                .map(|num| if i % 2 != 0 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| sum + num)
        });
    match digit_sum {
        Some(sum) => len > 1 && sum % 10 == 0,
        None => false,
    }
}
