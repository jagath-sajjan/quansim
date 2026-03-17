mod sim;
mod stats;
mod circuit;

use circuit::builder::CircuitBuilder;
use sim::measure::measure;
use sim::probabilities::probabilities;

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
}
