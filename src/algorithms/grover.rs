use std::f64::consts::PI;
use crate::circuit::builder::CircuitBuilder;
use crate::sim::state::QuantumState;
use crate::sim::probabilities::probabilities;

fn multi_controlled_z(builder: &mut CircuitBuilder, controls: &[usize], target: usize) {
    match controls.len() {
        1 => builder.cz(controls[0], target),
        2 => {
            builder.h(target);
            builder.toffoli(controls[0], controls[1], target);
            builder.h(target);
        }
        _ => builder.cz(controls[0], target),
    }
}

pub fn grover_oracle(builder: &mut CircuitBuilder, target: usize, n_qubits: usize) {
    for q in 0..n_qubits {
        if (target >> (n_qubits - 1 - q)) & 1 == 0 {
            builder.x(q);
        }
    }

    let controls: Vec<usize> = (0..n_qubits - 1).collect();
    let last = n_qubits - 1;
    multi_controlled_z(builder, &controls, last);

    for q in 0..n_qubits {
        if (target >> (n_qubits - 1 - q)) & 1 == 0 {
            builder.x(q);
        }
    }
}

pub fn grover_diffuser(builder: &mut CircuitBuilder, n_qubits: usize) {
    for q in 0..n_qubits {
        builder.h(q);
        builder.x(q);
    }

    let controls: Vec<usize> = (0..n_qubits - 1).collect();
    let last = n_qubits - 1;
    multi_controlled_z(builder, &controls, last);

    for q in 0..n_qubits {
        builder.x(q);
        builder.h(q);
    }
}

pub fn grover(n_qubits: usize, target: usize) -> (QuantumState, usize) {
    let iterations = ((PI / 4.0) * (2.0_f64.powi(n_qubits as i32)).sqrt()).round() as usize;
    let iterations = iterations.max(1);

    let mut builder = CircuitBuilder::new(n_qubits);

    for q in 0..n_qubits {
        builder.h(q);
    }

    for _ in 0..iterations {
        grover_oracle(&mut builder, target, n_qubits);
        grover_diffuser(&mut builder, n_qubits);
    }

    let state = builder.run();
    (state, iterations)
}

pub fn grover_result(state: &QuantumState) -> usize {
    let probs = probabilities(state);
    probs
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}
