#![allow(warnings)]

mod sim;
mod ui;
mod stats;
mod circuit;

use rayon::result;
use sim::state::QuantumState;
use sim::gates::*;
use sim::measure::*;

use crate::sim::measure;

fn main() {
    let mut state = QuantumState::new(2);

    hadmard(&mut state, 0);
    cnot(&mut state, 0, 1);

    println!("State:");
    state.print_state();

    let result = measure(&state);

    println!("\nMeasurement result: {:02b}", result);
}
