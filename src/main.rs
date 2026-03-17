mod sim;
mod stats;
mod circuit;

use circuit::builder::CircuitBuilder;
use sim::measure::measure;
use sim::probabilities::probabilities;
use sim::measure::measure_many;
use stats::histogram::histogram;
use stats::ascii_histogram::print_histogram;

fn main() {

    let mut builder = CircuitBuilder::new(2);

    builder.h(0);
    builder.cx(0, 1);

    println!("Circuit:");
    println!("{}", builder.circuit().draw());

    let state = builder.run();

    println!("Quantum State:");
    for (i, amp) in state.amplitudes.iter().enumerate() {
        println!("|{:02b}> {}+{}i", i, amp.re, amp.im);
    }

    println!("\nProbabilities:");
    let probs = probabilities(&state);

    for (i, p) in probs.iter().enumerate() {
        println!("|{:02b}> {:.3}", i, p);
    }

    let result = measure(&state);

    println!("\nMeasurement result: {:02b}", result);

    let shots = 1000;
    let results = measure_many(&state, shots);
    let counts = histogram(&results);

    print_histogram(&counts, shots);
}
