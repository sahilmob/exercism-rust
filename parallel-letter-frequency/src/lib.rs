use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();

    if input.is_empty() {
        return result;
    }

    let (tx, rx) = mpsc::channel();

    // Determine how many chunks to process
    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    // Create threads
    let handles: Vec<_> = input
        .chunks(chunk_size)
        .map(|chunk| {
            let tx = tx.clone();
            // Convert borrowed strings to owned strings
            let chunk = chunk.join("").to_string();

            thread::spawn(move || {
                let mut local_result: HashMap<char, usize> = HashMap::new();
                chunk.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                    *local_result
                        .entry(c.to_lowercase().next().unwrap())
                        .or_insert(0) += 1;
                });
                tx.send(local_result).unwrap();
            })
        })
        .collect();

    // Drop the original sender to prevent deadlock
    drop(tx);

    // Collect results from all threads
    while let Ok(local_map) = rx.recv() {
        for (key, value) in local_map {
            *result.entry(key).or_insert(0) += value;
        }
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    result
}
