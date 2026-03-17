use std::collections::HashMap;

pub fn histogram(results: &[usize]) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();

    for r in results {
        *counts.entry(*r).or_insert(0) += 1;
    }

    counts
}
