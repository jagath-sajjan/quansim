mod sim;
mod stats;
mod circuit;

use std::result;

use circuit::builder::CircuitBuilder;
use sim::measure::measure;
use sim::probabilities::probabilities;
use sim::shots::run_shots;

use crate::sim::state;

fn main() {
    let mut circuit = CircuitBuilder::new(2);

    circuit.h(0);
    circuit.cx(0, 1);

    let state = circuit.run();

    println!("Quantum State:");
    for (i, amp) in state.amplitudes.iter().enumerate() {
        println!("|{:02b}> {}", i, amp);
    }

    println!("\nProbabilities:");
    let probs = probabilities(&state);

    for (i, p) in probs.iter().enumerate() {
        println!("|{:02b}> {:.3}", i, p);
    }

    println!("\nShot Sampling");
    let results = run_shots(&state, 1000);

    for (state, count) in results {
        println!("|{:02b}> {}", state, count);
    }

    let result = measure(&state);

    println!("\nMeasurement result: {:02b}", result);
}
