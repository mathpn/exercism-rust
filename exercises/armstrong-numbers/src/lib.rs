pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let n_digits = digits.len() as u32;
    let sum: u32 = digits.into_iter().map(|d| d.pow(n_digits)).sum();
    return sum == num;
}
