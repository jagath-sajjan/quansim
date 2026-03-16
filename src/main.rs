mod sim;
mod stats;
mod circuit;

use circuit::builder::CircuitBuilder;
use sim::measure::measure;

fn main() {

    let mut circuit = CircuitBuilder::new(2);

    circuit.h(0);
    circuit.cx(0,1);

    let state = circuit.run();

    println!("Quantum State:");

    for (i, amp) in state.amplitudes.iter().enumerate() {
        println!("|{:02b}> {}", i, amp);
    }

    let result = measure(&state);

    println!("\nMeasurement result: {:02b}", result);
}
