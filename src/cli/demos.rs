use crate::circuit::builder::CircuitBuilder;
use crate::circuit::qasm::to_qasm;
use crate::sim::measure::measure_many;
use crate::sim::probabilities::probabilities;
use crate::stats::histogram::histogram;
use crate::stats::ascii_histogram::print_histogram;
use crate::algorithms::qft::qft;
use crate::algorithms::grover::{grover, grover_result};

pub fn run(args: &[String]) {
    match args.get(0).map(|s| s.as_str()) {
        Some("bell") => demo_bell(),
        Some("ghz") => demo_ghz(args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3)),
        Some("grover") => {
            let n = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3);
            let t = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);
            demo_grover(n, t);
        }
        Some("qft") => demo_qft(args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3)),
        Some("teleport") => demo_teleport(),
        _ => {
            println!("Available Demos:");
            println!(" cargo run -- demo bell");
            println!(" cargo run -- demo ghz <n_qubits>");
            println!(" cargo run -- demo grover <n_qubits> <target>");
            println!(" cargo run -- demo qft <n_qubits>");
            println!(" cargo run -- demo teleport");
        }
    }
}

fn demo_bell() {
    println!("BELL STATE");
    println!("Two Qubits Maximally Entangled, Measuring One Collapses The Other");
    println!("Only |00> & |11> Are Possible, Never |01> or |10>\n");

    let mut b = CircuitBuilder::new(2);
    b.h(0);
    b.cx(0, 1);

    println!("{}", b.circuit().draw());

    let state = b.run();
    let probs = probabilities(&state);
    for (i, p) in probs.iter().enumerate() {
        if *p > 1e-10 {
            println!("|{:02b}> {:.4}", i, p);
        }
    }

    println!();
    let results = measure_many(&state, 1000);
    let counts = histogram(&results);
    print_histogram(&counts, 1000);
    println!("\nQASM:\n{}", to_qasm(b.circuit()));
}

fn demo_ghz(n: usize) {
    println!("GHZ State ({} qubits)", n);
    println!("Generalised Bell State, All Qubits Entangled Togather\n");

    let mut b = CircuitBuilder::new(n);
    b.h(0);
    for i in 1..n {
        b.cx(0, i);
    }

    println!("{}", b.circuit().draw());

    let state = b.run();
    let probs = probabilities(&state);
    for (i, p) in probs.iter().enumerate() {
        if *p > 1e-10 {
            println!("|{:0width$b}> {:.4}", i, p, width = n);
        }
    }

    println!();
    let results = measure_many(&state, 1000);
    let counts = histogram(&results);
    print_histogram(&counts, 1000);
}

fn demo_grover(n: usize, target: usize) {
    println!("Grover Search ({} qubits, target={})", n, target);
    println!("Searches {} States In ~{} Iterations vs {} classical",
        1 << n,
        (((std::f64::consts::PI / 4.0) * (1u64 << n) as f64).sqrt()) as usize,
        1 << n
    );
    println!();

    let (state, iters) = grover(n, target);
    let found = grover_result(&state);

    println!("iterstions used: {}", iters);
    println!("found: {} (binary: {:0width$b})", found, found, width = n);
    println!("correct: {}\n", if found == target { "yes" } else { "no" });

    let probs = probabilities(&state);
    for (i, p) in probs.iter().enumerate() {
        if *p > 0.01 {
            println!("|{:0width$b}> {:.4} {}", i, p, if i == target { "<== target" } else { " " }, width =n );
        }
    }

    println!();
    let results = measure_many(&state, 1000);
    let counts = histogram(&results);
    print_histogram(&counts, 1000);
}

fn demo_qft(n: usize) {
    println!("Quantum Fourier Transform ({} qubits)", n);
    println!("Quantum Analouge OF FFT, Transforms Amplitudes Into Frequency Domain");
    println!("Equal Probabilities Are Expected: information is encoded in phases\n");

    let mut b = CircuitBuilder::new(n);
    b.x(0);
    qft(&mut b, &(0..n).collect::<Vec<_>>());

    println!("{}", b.circuit().draw());
    println!("depth: {} gates: {}\n", b.depth(), b.gate_count());

    let state = b.run();
    let probs = probabilities(&state);
    for (i, p) in probs.iter().enumerate() {
        println!("|{:0width$b}> {:.4}", i, p, width = n);
    }
} 

fn demo_teleport() {
    println!("Quantum Teleportation");
    println!("Transfers Quantum State From Qubit 0 To Qubit 2 Using Entanglement");
    println!("Qubit 0: message qubit 1+2: entangled pair\n");

    let mut b = CircuitBuilder::new(3);

    println!("preparing message qubit in |+> state");
    b.h(0);

    println!("creating bell pair on qubits 1 & 2");
    b.h(1);
    b.cx(1, 2);

    println!("bell measurement on qubits 0 & 1");
    b.cx(0, 1);
    b.h(0);

    println!("applying corrections to qubit 2");
    b.cx(1, 2);
    b.cz(0, 2);

    println!("\n{}", b.circuit().draw());

    let state = b.run();
    let probs = probabilities(&state);
    println!("final state probabilities");
    for (i, p) in probs.iter().enumerate() {
        if *p > 1e-10 {
            println!("|{:03b}> {:.4}", i, p);
        }
    }
}
