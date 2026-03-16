use super::state::QuantumState;
use num_complex::Complex64;

pub fn hadamard(state: &mut QuantumState, target: usize) {
    let n = state.amplitudes.len();
    let step = 1 << target;

    let inv_sqrt2 = 1.0 / (2.0_f64).sqrt();

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            let a = state.amplitudes[i + j];
            let b = state.amplitudes[i + j + step];

            state.amplitudes[i + j] = (a + b) * inv_sqrt2;
            state.amplitudes[i + j + step] = (a - b) * inv_sqrt2;
        }
    }
}

pub fn x(state: &mut QuantumState, target: usize) {
    let n = state.amplitudes.len();
    let step = 1 << target;

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            state.amplitudes.swap(i + j, i + j + step);
        }
    }
}

pub fn cnot(state: &mut QuantumState, control: usize, target: usize) {
    let n = state.amplitudes.len();

    for i in 0..n {
        if ((i >> control) & 1) == 1 {
            let flipped = i ^ (1 << target);

            if i < flipped {
                state.amplitudes.swap(i, flipped);
            }
        }
    }
}
