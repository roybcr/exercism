use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunks = input.chunks(worker_count);
    let mut handles = vec![];

    for chunk in chunks {
        let c = chunk
            .into_iter()
            .flat_map(|s| s.chars().filter(|c| c.is_alphabetic()))
            .flat_map(|s| s.to_lowercase())
            .collect();

        handles.push(thread::spawn(move || _frequency(c)));
    }

    let mut result = HashMap::new();
    for handle in handles {
        for (c, count) in handle.join().unwrap() {
            *result.entry(c).or_default() += count;
        }
    }

    result
}

fn _frequency(input: String) -> HashMap<char, usize> {
    input
        .chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, usize>, curr| {
            *acc.entry(curr).or_default() += 1;
            acc
        })
}
