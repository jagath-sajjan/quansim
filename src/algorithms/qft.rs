use std::f64::consts::PI;
use crate::circuit::builder::CircuitBuilder;

pub fn qft(builder: &mut CircuitBuilder, qubits: &[usize]) {
    let n = qubits.len();

    for i in 0..n {
        builder.h(qubits[i]);
        for j in (i + 1)..n {
            let k = (j - i + 1) as f64;
            let theta = PI / 2.0_f64.powi((k - 1.0) as i32);
            builder.crz(qubits[i], qubits[j], theta);
        }
    }

    for i in 0..(n / 2) {
        builder.swap(qubits[i], qubits[n - 1 - i]);
    }
}

pub fn iqft(builder: &mut CircuitBuilder, qubits: &[usize]) {
    let n = qubits.len();

    for i in 0..(n / 2) {
        builder.swap(qubits[i], qubits[n - 1 - i]);
    }

    for i in (0..n).rev() {
        for j in ((i + 1)..n).rev() {
            let k = (j - i + 1) as f64;
            let theta = -PI / 2.0_f64.powi((k - 1.0) as i32);
            builder.crz(qubits[i], qubits[j], theta);
        }
        builder.h(qubits[i]);
    }
}
