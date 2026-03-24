mod sim;
mod stats;
mod circuit;
mod algorithms;

use circuit::builder::CircuitBuilder;
use circuit::qasm::to_qasm;
use sim::measure::{measure, measure_many};
use sim::probabilities::probabilities;
use sim::fidelity::{fidelity, DensityMatrix};
use sim::noise::NoiseModel;
use sim::expectation::expect_z;
use stats::histogram::histogram;
use stats::ascii_histogram::print_histogram;
use algorithms::qft::qft;
use algorithms::grover::{grover, grover_result};

fn main() {
    println!("Bell State");
    let mut builder = CircuitBuilder::new(2);
    builder.h(0);
    builder.cx(0, 1);
    println!("{}", builder.circuit().draw());
    let state = builder.run();
    for (i, amp) in state.amplitudes.iter().enumerate() {
        println!("|{:02b}> {}+{}i", i, amp.re, amp.im);
    }
    let probs = probabilities(&state);
    for (i, p) in probs.iter().enumerate() {
        println!("|{:02b}> {:.3}", i, p);
    }
    println!("result: {:02b}", measure(&state));
    let results = measure_many(&state, 1000);
    let counts = histogram(&results);
    print_histogram(&counts, 1000);

    println!("\nQASM Export");
    println!("{}", to_qasm(builder.circuit()));

    println!("\nQFT (3 qubits)");
    let mut qft_builder = CircuitBuilder::new(3);
    qft_builder.x(0);
    qft(&mut qft_builder, &[0, 1, 2]);
    println!("{}", qft_builder.circuit().draw());
    println!("depth: {}  gates: {}", qft_builder.depth(), qft_builder.gate_count());
    let qft_state = qft_builder.run();
    let qft_probs = probabilities(&qft_state);
    for (i, p) in qft_probs.iter().enumerate() {
        if *p > 1e-6 {
            println!("|{:03b}> {:.4}", i, p);
        }
    }

    println!("\nGrover Search (3 qubits, target=5)");
    let (grover_state, iters) = grover(3, 5);
    println!("iterations: {}", iters);
    let found = grover_result(&grover_state);
    println!("found: {} (expected: 5)", found);
    let grover_probs = probabilities(&grover_state);
    for (i, p) in grover_probs.iter().enumerate() {
        if *p > 0.01 {
            println!("|{:03b}> {:.4}", i, p);
        }
    }

    println!("\nFidelity + Density Matrix");
    let mut circ_a = CircuitBuilder::new(1);
    circ_a.h(0);
    let state_a = circ_a.run();

    let mut circ_b = CircuitBuilder::new(1);
    circ_b.h(0);
    let state_b = circ_b.run();

    println!("fidelity(H|0>, H|0>): {:.6}", fidelity(&state_a, &state_b));
    let dm = DensityMatrix::from_state(&state_a);
    println!("trace: {:.4}  purity: {:.4}", dm.trace(), dm.purity());
    dm.print();

    println!("\nNoise Model");
    let noise = NoiseModel::new(0.02, 0.01, 0.01);
    let mut noisy = CircuitBuilder::new(2);
    noisy.h(0);
    noisy.cx(0, 1);
    let mut noisy_state = noisy.run();
    println!("<Z> qubit 0 before noise: {:.4}", expect_z(&noisy_state, 0));
    noise.apply(&mut noisy_state);
    println!("<Z> qubit 0 after noise:  {:.4}", expect_z(&noisy_state, 0));
}
