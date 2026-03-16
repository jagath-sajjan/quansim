#![allow(warnings)]

mod sim;
mod ui;
mod stats;
mod circuit;

use sim::state::QuantumState;
use sim::gates::*;

fn main() {
    let mut state = QuantumState::new(2);

    println!("Initial state:");
    state.print_state();

    println!("\nApply H on qubit 0");
    hadmard(&mut state, 0);
    state.print_state();

    println!("\nApply CNOT (0 -> 1)");
    cnot(&mut state, 0, 1);
    state.print_state();
}
