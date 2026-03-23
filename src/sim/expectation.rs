use super::state::QuantumState;

pub fn expect_z(state: &QuantumState, qubit: usize) -> f64 {
    state.amplitudes.iter().enumerate().fold(0.0, |acc, (i, amp)| {
        let bit = (i >> qubit) & 1;
        let sign = if bit == 0 { 1.0 } else { -1.0 };
        acc + sign * amp.norm_sqr()
    })
}

pub fn expect_x(state: &QuantumState, qubit: usize) -> f64 {
    let n = state.amplitudes.len();
    let step = 1 << qubit;
    let mut result = 0.0;

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            let a = state.amplitudes[i + j];
            let b = state.amplitudes[i + j + step];
            result += 2.0 * (a.conj() * b).re;
        }
    }

    result
}

pub fn expect_y(state: &QuantumState, qubit: usize) -> f64 {
    let n = state.amplitudes.len();
    let step = 1 << qubit;
    let mut result = 0.0;

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            let a = state.amplitudes[i + j];
            let b = state.amplitudes[i + j + step];
            result += 2.0 * (a.conj() * b).im;
        }
    }

    result
}
