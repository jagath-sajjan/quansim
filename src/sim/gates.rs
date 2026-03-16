use num_complex::Complex64;
use std::f64::consts::SQRT_2;

use super::state::QuantumState;

pub fn hadmard(state: &mut QuantumState, target: usize) {
    let size = state.amplitudes.len();
    let step = 1 << target;

    for i in (0..size).step_by(step * 2) {
        for j in 0..step {
            let a = state.amplitudes[i + j];
            let b = state.amplitudes[i + j + step];

            state.amplitudes[i + j] = (a + b) / SQRT_2;
            state.amplitudes[i + j + step] = (a - b) / SQRT_2;

        }
    }
}

pub fn pauli_x(state: &mut QuantumState, target: usize) {
    let size = state.amplitudes.len();
    let step = 1 << target;

    for i in (0..size).step_by(step * 2) {
        for j in 0..step {
            state.amplitudes.swap(i + j, i + j + step);
        }
    }
}

pub fn cnot(state: &mut QuantumState, control: usize, target: usize) {
    let size = state.amplitudes.len();

    for i in 0..size {
        if ((i >> control) & 1) == 1 {
            let flipped = i ^ (1 << target);

            if i < flipped {
                state.amplitudes.swap(i, flipped);
            }
        }
    }
}
