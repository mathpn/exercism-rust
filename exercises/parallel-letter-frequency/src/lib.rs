use std::collections::HashMap;
use std::thread;
use std::cmp::min;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut global_counter = HashMap::new();

    let input = input.join("");
    if input.is_empty() { return global_counter; }

    let mut threads = Vec::new();
    let chunk_size = input.len() / worker_count + 1;
    let n_chunks = input.len() / chunk_size + 1;

    (0..n_chunks).for_each(|i| {
        let end = min(input.len(), (i + 1) * chunk_size);
        let text = input[i*chunk_size..end].to_string();
        threads.push(thread::spawn(move || count_chars(text)));
    });

    for handler in threads {
        for (char, count) in handler.join().unwrap() {
            *global_counter.entry(char).or_insert(0) += count;
        }
    }
    return global_counter;

}

fn count_chars(text: String) -> HashMap<char, usize> {
    let mut counter: HashMap<char, usize> = HashMap::new();
    let lower_text = text.to_lowercase();
    for char in lower_text.chars() {
        if !char.is_alphabetic() { continue; };
        *counter.entry(char).or_insert(0) += 1;
    };
    return counter;
}

