use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_counter: HashMap<&str, i32> = HashMap::new();
    for word in magazine {
        *magazine_counter.entry(word).or_insert(0) += 1;
    }
    for word in note {
        *magazine_counter.entry(word).or_default() -= 1;
        match magazine_counter.get(word) {
            Some(n) if n >= &0 => continue,
            _ => return false,
        }
    }
    return true;
}
