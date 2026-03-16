use sim::state::QuantumState;

mod sim;
mod ui;
mod stats;
mod circuit;

fn main() {
    let state = QuantumState::new(3);

    println!("qubits: {}", state.qubits);
    println!("dimension: {}", state.dimension());
    println!("memory: {}", state.memory_bytes());

    state.print_state();
}
