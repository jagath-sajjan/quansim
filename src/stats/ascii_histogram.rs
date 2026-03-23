use std::{collections::HashMap, usize};

pub fn print_histogram(counts: &HashMap<usize, usize>, shots: usize) {
    let max = counts.values().max().unwrap_or(&1);
    let mut sorted: Vec<(&usize, &usize)> = counts.iter().collect();
    sorted.sort_by_key(|(state, _)| *state);

    for (state, count) in sorted {
        let bar_len = (50 * count) / max;
        let bar = "█".repeat(bar_len);
        println!("|{:02b}> {:<50} {}", state, bar, count);
    }

    println!("shots: {}", shots);
}
